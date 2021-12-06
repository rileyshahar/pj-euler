//! Wrappers around the `num` crate for generic number traits.

use num::Num;

#[inline]
pub fn _0<N: Num>() -> N {
    N::zero()
}

#[inline]
pub fn _1<N: Num>() -> N {
    N::one()
}

#[inline]
pub fn _2<N: Num>() -> N {
    _1::<N>() + _1()
}

#[inline]
pub fn _3<N: Num>() -> N {
    _2::<N>() + _1()
}
