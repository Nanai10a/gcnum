
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
