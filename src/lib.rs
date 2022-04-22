#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Usize<const N: usize>;

macro_rules! try_from_impls {
    ($($ty:ty)*) => {$(
        impl<const N: usize> TryFrom<$ty> for Usize<N> {
            type Error = ();

            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                if value == N as $ty {
                    Ok(Usize::<N>)
                } else {
                    Err(())
                }
            }
        }
    )*};
}

try_from_impls! { u8 u16 u32 u64 u128 usize }

macro_rules! from_impls {
    ($($ty:ty)*) => {$(
        impl<const N: usize> From<Usize<N>> for $ty {
            fn from(_: Usize<N>) -> Self { N as $ty }
        }
    )*};
}

from_impls! { u8 u16 u32 u64 u128 usize }

impl<const N: usize> PartialEq<usize> for Usize<N> {
    fn eq(&self, other: &usize) -> bool { Usize::<N>::try_from(*other).is_ok() }
}

impl<const N: usize> PartialEq<Usize<N>> for usize {
    fn eq(&self, _: &Usize<N>) -> bool { Usize::<N>::try_from(*self).is_ok() }
}

#[test]
fn check_value() {
    let pnum = 771usize;
    let gnum = Usize::<771>;

    assert_eq! { pnum, gnum };
    assert_eq! { gnum, pnum };

    let _: usize = gnum.into();
    let _: Usize<771> = pnum.try_into().unwrap();
}

// serde impls

use core::fmt;

use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

impl<const N: usize> Serialize for Usize<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_u64(Into::<usize>::into(*self) as u64)
    }
}

struct UsizeVisitor<const N: usize>;

macro_rules! into_or_fallback {
    ($fn:ident($ty:ty) | -> $nfn:ident($nty:ty)) => {
        fn $fn<E: de::Error>(self, v: $ty) -> Result<Self::Value, E> {
            if <$ty>::BITS == usize::BITS {
                TryInto::try_into(v).map_err(|_| {
                    de::Error::invalid_value(de::Unexpected::Unsigned(v.into()), &self)
                })
            } else {
                self.$nfn(v as $nty)
            }
        }
    };
    ($fn:ident($ty:ty) |) => {
        fn $fn<E: de::Error>(self, v: $ty) -> Result<Self::Value, E> {
            if <$ty>::BITS == usize::BITS {
                TryInto::try_into(v).map_err(|_| {
                    de::Error::invalid_value(de::Unexpected::Unsigned(v.into()), &self)
                })
            } else {
                unreachable!();
            }
        }
    };
}

impl<'de, const N: usize> Visitor<'de> for UsizeVisitor<N> {
    type Value = Usize<N>;

    into_or_fallback! { visit_u8(u8) |-> visit_u16(u16) }

    into_or_fallback! { visit_u16(u16) |-> visit_u32(u32) }

    into_or_fallback! { visit_u32(u32) |-> visit_u64(u64) }

    into_or_fallback! { visit_u64(u64) | }

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "number must be {}", N)
    }
}

impl<'de, const N: usize> Deserialize<'de> for Usize<N> {
    fn deserialize<D>(deserliazer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        deserliazer.deserialize_u8(UsizeVisitor)
    }
}

#[test]
fn check_serde_struct() {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct Struct {
        thirty_three: Usize<33>,
        four: Usize<4>,
    }

    let json = r#"{"thirty_three":33,"four":4}"#;
    let data = Struct {
        thirty_three: Usize::<33>,
        four: Usize::<4>,
    };

    let cjson = serde_json::from_str(json).unwrap();
    let cdata = serde_json::to_string(&data).unwrap();

    assert_eq! { dbg!(json), dbg!(cdata) }
    assert_eq! { dbg!(data), dbg!(cjson) }
}

#[test]
fn check_serde_enum() {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    // warning: if reverses to define sequence of `Additional` & `Minimal` , serde
    // always deserialize `Addtional` scheme to `Minimal` one.
    enum Enum {
        Turple(Usize<1>, Usize<2>, Usize<3>),
        Additional { must: Usize<0>, optional: Usize<1> },
        Minimal { must: Usize<0> },
        None,
    }

    let tjson = "[1,2,3]";
    let mjson = r#"{"must":0}"#;
    let ajson = r#"{"must":0,"optional":1}"#;
    let njson = "null";

    let tdata = Enum::Turple(Usize::<1>, Usize::<2>, Usize::<3>);
    let mdata = Enum::Minimal { must: Usize::<0> };
    let adata = Enum::Additional {
        must: Usize::<0>,
        optional: Usize::<1>,
    };
    let ndata = Enum::None;

    let pairs = [
        (tjson, tdata),
        (mjson, mdata),
        (ajson, adata),
        (njson, ndata),
    ];

    let jsons = pairs
        .as_ref()
        .into_iter()
        .map(|(j, e)| (j, serde_json::to_string(&e).unwrap()));
    let datas = pairs
        .as_ref()
        .into_iter()
        .map(|(j, e)| (serde_json::from_str::<Enum>(j).unwrap(), e));

    for (j, e) in jsons {
        assert_eq! { dbg!(j), dbg!(&e) }
    }

    for (j, e) in datas {
        assert_eq! { dbg!(j), *dbg!(e) }
    }
}

// ops impls

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

macro_rules! ops_impl {
    ($ty:tt :: $fn:tt : $op:tt) => {
        impl<const LHS: usize, const RHS: usize> $ty<Usize<RHS>> for Usize<LHS>
        where [(); LHS $op RHS]:
        {
            type Output = Usize<{ LHS $op RHS }>;

            fn $fn(self, _: Usize<RHS>) -> Self::Output { Usize::<{ LHS $op RHS }> }
        }
    };
}

ops_impl! { Add :: add : + }
ops_impl! { BitAnd :: bitand : & }
ops_impl! { BitOr :: bitor : | }
ops_impl! { BitXor :: bitxor : ^ }
ops_impl! { Div :: div : / }
ops_impl! { Mul :: mul : * }
ops_impl! { Rem :: rem : % }
ops_impl! { Shl :: shl : << }
ops_impl! { Shr :: shr : >> }
ops_impl! { Sub :: sub : - }

impl<const N: usize> Not for Usize<N>
where [(); !N]:
{
    type Output = Usize<{ !N }>;

    fn not(self) -> Self::Output { Usize::<{ !N }> }
}
