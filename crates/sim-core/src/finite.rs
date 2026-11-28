//! ARC-023: every state `f64` is finite after every stage (debug and test builds). A serde
//! serializer walks each block and reports the path of the first non-finite value.
use crate::pipeline::Stage;
use crate::state::World;
use serde::ser::{self, Serialize};
use std::fmt;

/// Panics naming the stage, block and field path of the first non-finite `f64`.
pub(crate) fn check_world(world: &World, stage: Stage) {
    check(stage, "polities", &world.polities);
    check(stage, "econ", &world.econ);
    check(stage, "gov", &world.gov);
    check(stage, "rnd", &world.rnd);
    check(stage, "force", &world.force);
    check(stage, "resources", &world.resources);
    check(stage, "space", &world.space);
    check(stage, "engagements", &world.engagements);
    check(stage, "beliefs", world.beliefs());
    check(stage, "events", &world.events);
    #[cfg(any(test, feature = "stubs"))]
    check(stage, "stub", &world.stub);
}

fn check<T: Serialize + ?Sized>(stage: Stage, block: &str, value: &T) {
    let mut walker = Walker { path: Vec::new() };
    if let Err(NonFinite(path)) = value.serialize(&mut walker) {
        panic!("non-finite f64 after stage {stage:?} in block `{block}` at {path}");
    }
}

#[derive(Debug)]
struct NonFinite(String);
impl fmt::Display for NonFinite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for NonFinite {}
impl ser::Error for NonFinite {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        NonFinite(msg.to_string())
    }
}

struct Walker {
    path: Vec<String>,
}

impl Walker {
    fn float(&self, v: f64) -> Result<(), NonFinite> {
        if v.is_finite() {
            Ok(())
        } else {
            Err(NonFinite(format!("{}= {v}", self.path.join(""))))
        }
    }
    fn nested<T: Serialize + ?Sized>(&mut self, seg: String, v: &T) -> Result<(), NonFinite> {
        self.path.push(seg);
        let r = v.serialize(&mut *self);
        self.path.pop();
        r
    }
}

type R = Result<(), NonFinite>;

impl<'a> ser::Serializer for &'a mut Walker {
    type Ok = ();
    type Error = NonFinite;
    type SerializeSeq = Seq<'a>;
    type SerializeTuple = Seq<'a>;
    type SerializeTupleStruct = Seq<'a>;
    type SerializeTupleVariant = Seq<'a>;
    type SerializeMap = Seq<'a>;
    type SerializeStruct = Seq<'a>;
    type SerializeStructVariant = Seq<'a>;

    fn serialize_f64(self, v: f64) -> R {
        self.float(v)
    }
    fn serialize_f32(self, v: f32) -> R {
        self.float(f64::from(v))
    }
    fn serialize_bool(self, _: bool) -> R {
        Ok(())
    }
    fn serialize_i8(self, _: i8) -> R {
        Ok(())
    }
    fn serialize_i16(self, _: i16) -> R {
        Ok(())
    }
    fn serialize_i32(self, _: i32) -> R {
        Ok(())
    }
    fn serialize_i64(self, _: i64) -> R {
        Ok(())
    }
    fn serialize_u8(self, _: u8) -> R {
        Ok(())
    }
    fn serialize_u16(self, _: u16) -> R {
        Ok(())
    }
    fn serialize_u32(self, _: u32) -> R {
        Ok(())
    }
    fn serialize_u64(self, _: u64) -> R {
        Ok(())
    }
    fn serialize_char(self, _: char) -> R {
        Ok(())
    }
    fn serialize_str(self, _: &str) -> R {
        Ok(())
    }
    fn serialize_bytes(self, _: &[u8]) -> R {
        Ok(())
    }
    fn serialize_none(self) -> R {
        Ok(())
    }
    fn serialize_some<T: Serialize + ?Sized>(self, v: &T) -> R {
        v.serialize(self)
    }
    fn serialize_unit(self) -> R {
        Ok(())
    }
    fn serialize_unit_struct(self, _: &'static str) -> R {
        Ok(())
    }
    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> R {
        Ok(())
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _: &'static str, v: &T) -> R {
        v.serialize(self)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        _: u32,
        var: &'static str,
        v: &T,
    ) -> R {
        self.nested(format!("::{var}"), v)
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Seq<'a>, NonFinite> {
        Ok(Seq { w: self, i: 0 })
    }
    fn serialize_tuple(self, _: usize) -> Result<Seq<'a>, NonFinite> {
        Ok(Seq { w: self, i: 0 })
    }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Seq<'a>, NonFinite> {
        Ok(Seq { w: self, i: 0 })
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Seq<'a>, NonFinite> {
        Ok(Seq { w: self, i: 0 })
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Seq<'a>, NonFinite> {
        Ok(Seq { w: self, i: 0 })
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Seq<'a>, NonFinite> {
        Ok(Seq { w: self, i: 0 })
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Seq<'a>, NonFinite> {
        Ok(Seq { w: self, i: 0 })
    }
}

/// Compound walker: elements are labeled `[i]`, struct fields `.name`.
struct Seq<'a> {
    w: &'a mut Walker,
    i: usize,
}

impl Seq<'_> {
    fn element<T: Serialize + ?Sized>(&mut self, v: &T) -> R {
        let seg = format!("[{}]", self.i);
        self.i += 1;
        self.w.nested(seg, v)
    }
}

macro_rules! seq_impl {
    ($($tr:ident :: $m:ident),*) => {$(
        impl ser::$tr for Seq<'_> {
            type Ok = ();
            type Error = NonFinite;
            fn $m<T: Serialize + ?Sized>(&mut self, v: &T) -> R {
                self.element(v)
            }
            fn end(self) -> R {
                Ok(())
            }
        }
    )*};
}
seq_impl!(
    SerializeSeq::serialize_element,
    SerializeTuple::serialize_element,
    SerializeTupleStruct::serialize_field,
    SerializeTupleVariant::serialize_field
);

impl ser::SerializeMap for Seq<'_> {
    type Ok = ();
    type Error = NonFinite;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, k: &T) -> R {
        self.w.nested("{key}".into(), k)
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, v: &T) -> R {
        self.element(v)
    }
    fn end(self) -> R {
        Ok(())
    }
}

macro_rules! struct_impl {
    ($($tr:ident),*) => {$(
        impl ser::$tr for Seq<'_> {
            type Ok = ();
            type Error = NonFinite;
            fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, v: &T) -> R {
                self.w.nested(format!(".{key}"), v)
            }
            fn end(self) -> R {
                Ok(())
            }
        }
    )*};
}
struct_impl!(SerializeStruct, SerializeStructVariant);
