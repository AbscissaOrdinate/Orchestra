//! Typed dense indices (WS-02 §3.2). Never reused within a world; dead entities keep their slot.
use serde::{Deserialize, Serialize};

macro_rules! id {
    ($($(#[$m:meta])* $name:ident($t:ty);)*) => {$(
        $(#[$m])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct $name(pub $t);
    )*};
}

id! {
    PolityId(u16);
    RegionId(u16);
    BodyId(u16);
    /// Sol graph node = (body, orbit regime); WS-08b §3.2.
    NodeId(u16);
    /// Content techs get dense indices at load, sorted by string ID.
    TechIdx(u32);
    /// Dense belief subject index; see `SubjectRange` (§3.2, ARC-016).
    SubjectIdx(u32);
    /// WS-11 tracked unit.
    UnitId(u32);
    /// WS-08b deposit.
    DepositId(u16);
    /// WS-08b space facility.
    FacilityId(u32);
    /// Months since scenario start. No wall clock.
    Tick(u32);
}

impl PolityId {
    pub fn index(self) -> usize {
        usize::from(self.0)
    }
}
