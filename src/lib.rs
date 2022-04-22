
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
