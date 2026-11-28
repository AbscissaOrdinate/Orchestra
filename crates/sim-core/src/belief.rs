//! Beliefs (WS-02 §3.2, ADR-0008): a 1–3 component Gaussian mixture on a declared scale, and the
//! per-polity `BeliefBlock` with one optional slice per subject range (v1.3, WS-05 P-02a).
use crate::ids::{PolityId, SubjectIdx};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Positive quantities use `Log`, probabilities `Logit`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scale {
    Linear,
    Log,
    Logit,
}

impl Scale {
    /// Natural units → this scale.
    pub fn to_scale(self, x: f64) -> f64 {
        match self {
            Scale::Linear => x,
            Scale::Log => x.ln(),
            Scale::Logit => (x / (1.0 - x)).ln(),
        }
    }
}

/// One mixture component, on the belief's scale.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Component {
    pub w: f64,
    pub mu: f64,
    pub sigma: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Belief {
    pub scale: Scale,
    /// Number of used components (1..=3); `comps[n..]` are zero.
    pub n: u8,
    pub comps: [Component; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeliefError {
    /// Not 1..=3 components.
    Count,
    /// Weights not ≥ 0 or not summing to 1 (within 1e-9).
    Weights,
    /// A μ or σ that is not finite, or σ ≤ 0.
    Sigma,
}

impl Belief {
    /// One component with `mu`, `sigma` on `scale`.
    pub fn normal(scale: Scale, mu: f64, sigma: f64) -> Self {
        let mut comps = [Component::default(); 3];
        comps[0] = Component { w: 1.0, mu, sigma };
        Self { scale, n: 1, comps }
    }

    pub fn mixture(scale: Scale, comps: &[Component]) -> Result<Self, BeliefError> {
        if !(1..=3).contains(&comps.len()) {
            return Err(BeliefError::Count);
        }
        if comps.iter().any(|c| c.w < 0.0)
            || (comps.iter().map(|c| c.w).sum::<f64>() - 1.0).abs() > 1e-9
        {
            return Err(BeliefError::Weights);
        }
        if comps
            .iter()
            .any(|c| !c.mu.is_finite() || !c.sigma.is_finite() || c.sigma <= 0.0)
        {
            return Err(BeliefError::Sigma);
        }
        let mut all = [Component::default(); 3];
        all[..comps.len()].copy_from_slice(comps);
        Ok(Self {
            scale,
            n: comps.len() as u8,
            comps: all,
        })
    }

    /// The used components.
    pub fn components(&self) -> &[Component] {
        &self.comps[..usize::from(self.n)]
    }

    /// Moment-matched mean in natural units.
    pub fn mean(&self) -> f64 {
        self.moments().0
    }

    /// Moment-matched σ in natural units.
    pub fn sigma(&self) -> f64 {
        let (m1, m2) = self.moments();
        (m2 - m1 * m1).max(0.0).sqrt()
    }

    /// First and second raw moments in natural units. Linear and Log are closed form; Logit has
    /// no closed form and uses a fixed trapezoid grid over ±10σ (deterministic, not exact).
    fn moments(&self) -> (f64, f64) {
        let per = |c: &Component| -> (f64, f64) {
            let v = c.sigma * c.sigma;
            match self.scale {
                Scale::Linear => (c.mu, v + c.mu * c.mu),
                Scale::Log => ((c.mu + v / 2.0).exp(), (2.0 * c.mu + 2.0 * v).exp()),
                Scale::Logit => logit_normal_moments(c.mu, c.sigma),
            }
        };
        self.components().iter().fold((0.0, 0.0), |(a, b), c| {
            let (m1, m2) = per(c);
            (a + c.w * m1, b + c.w * m2)
        })
    }

    /// Exact Bayesian update with a Gaussian observation: Kalman update per component plus
    /// likelihood reweighting. `value` is in natural units; `noise_sigma` is on the belief's scale.
    pub fn observe(&mut self, value: f64, noise_sigma: f64) {
        let z = self.scale.to_scale(value);
        let r2 = noise_sigma * noise_sigma;
        let n = usize::from(self.n);
        let mut log_w = [f64::NEG_INFINITY; 3];
        for (c, lw) in self.comps[..n].iter_mut().zip(&mut log_w) {
            let s2 = c.sigma * c.sigma;
            let pred = s2 + r2;
            *lw = c.w.ln() - 0.5 * (pred.ln() + (z - c.mu).powi(2) / pred);
            let gain = s2 / pred;
            c.mu += gain * (z - c.mu);
            c.sigma = (s2 * (1.0 - gain)).sqrt();
        }
        let max = log_w[..n].iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let total: f64 = log_w[..n].iter().map(|lw| (lw - max).exp()).sum();
        for (c, lw) in self.comps[..n].iter_mut().zip(log_w) {
            c.w = (lw - max).exp() / total;
        }
    }
}

/// E[s(X)], E[s(X)²] for X ~ N(mu, sigma²), s = logistic.
fn logit_normal_moments(mu: f64, sigma: f64) -> (f64, f64) {
    const STEPS: i32 = 160; // h = σ/8 over ±10σ
    let h = 20.0 / f64::from(STEPS);
    let (mut m1, mut m2) = (0.0, 0.0);
    for i in 0..=STEPS {
        let t = -10.0 + h * f64::from(i);
        let edge = if i == 0 || i == STEPS { 0.5 } else { 1.0 };
        let weight = edge * h * (-0.5 * t * t).exp() / (2.0 * std::f64::consts::PI).sqrt();
        let p = 1.0 / (1.0 + (-(mu + sigma * t)).exp());
        m1 += weight * p;
        m2 += weight * p * p;
    }
    (m1, m2)
}

/// A belief update request. Subsystems push these; only stage I1 applies them (ARC-014).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    pub polity: PolityId,
    pub subject: SubjectIdx,
    /// Natural units.
    pub value: f64,
    /// On the belief's scale.
    pub noise_sigma: f64,
    pub source: Cow<'static, str>,
}

/// One subsystem's contiguous subject range, recorded in `WorldHeader` (identical for every polity).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubjectRange {
    pub owner: Cow<'static, str>,
    pub start: u32,
    pub len: u32,
}

/// A polity's beliefs: one optional slice per `SubjectRange`. `None` for a slice = the polity
/// doesn't hold that range (e.g. a macro polity and the WS-05 range); `None` for an entry = the
/// subject is unknown to the polity. Storage is private: `get` reads, the I1 writer writes.
///
/// ```compile_fail
/// fn peek(b: &sim_core::BeliefBlock) { let _ = &b.ranges; } // ARC-016(c): storage is private
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeliefBlock {
    ranges: Vec<Option<Vec<Option<Belief>>>>,
    /// Range starts, parallel to `ranges`, so `get` can locate an omitted range.
    starts: Vec<u32>,
}

impl BeliefBlock {
    /// Built at worldgen from the header's ranges; `slices[i]` is `None` to omit range `i`.
    pub(crate) fn new(ranges: &[SubjectRange], slices: Vec<Option<Vec<Option<Belief>>>>) -> Self {
        debug_assert_eq!(ranges.len(), slices.len());
        Self {
            ranges: slices,
            starts: ranges.iter().map(|r| r.start).collect(),
        }
    }

    /// `None` if the subject is unknown, out of range, or its range is omitted.
    pub fn get(&self, s: SubjectIdx) -> Option<&Belief> {
        let (slice, offset) = self.locate(s)?;
        self.ranges[slice].as_ref()?.get(offset)?.as_ref()
    }

    fn locate(&self, s: SubjectIdx) -> Option<(usize, usize)> {
        let slice = self
            .starts
            .partition_point(|&start| start <= s.0)
            .checked_sub(1)?;
        Some((slice, (s.0 - self.starts[slice]) as usize))
    }

    /// The I1 writer (ARC-014). Observations of unknown subjects are ignored: discovery rules
    /// belong to each subsystem's spec.
    pub(crate) fn apply(&mut self, o: &Observation) {
        let Some((slice, offset)) = self.locate(o.subject) else {
            return;
        };
        if let Some(Some(Some(b))) = self.ranges[slice].as_mut().map(|v| v.get_mut(offset)) {
            b.observe(o.value, o.noise_sigma);
        }
    }

    /// Whether range `i` is stored (test-only introspection for ARC-016(b)).
    #[cfg(test)]
    pub(crate) fn holds(&self, i: usize) -> bool {
        self.ranges[i].is_some()
    }
}
