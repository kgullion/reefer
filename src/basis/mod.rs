pub mod add;
pub mod default;
pub mod display;
pub mod dual;
pub mod equality;
pub mod grade;
pub mod into;
pub mod mul;
pub mod neg;

use crate::{metric::Metric, ta};
use core::marker::PhantomData;
use typenum::{Bit, Unsigned};

// -------------------------------------
/// A Basis is a signed product of unit length basis vectors in a geometric algebra.
#[derive(Clone, Copy, Debug)]
pub struct Basis<U: Unsigned, M: Metric, S: Bit>(PhantomData<(U, M, S)>);

// the zero vector is a special case, every other basis is length 1
/// Represents the zero vector. You probably don't need to use this directly and got it by multiplying
/// two degenerate Basis elements. Mostly here for the compiler.
pub struct ZeroVect<M>(PhantomData<M>);

// -------------------------------------
// basis operations
// const new
impl<U: Unsigned, M: Metric, S: Bit> Basis<U, M, S> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<M: Metric> ZeroVect<M> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

pub trait BasisInfo {
    type Mask: Unsigned;
    type Metric: Metric;
    type Parity: Bit;
}
impl<M: Metric> BasisInfo for ZeroVect<M> {
    type Mask = typenum::U0;
    type Metric = ta![];
    type Parity = typenum::B0;
}
impl<U: Unsigned, M: Metric, S: Bit> BasisInfo for Basis<U, M, S> {
    type Mask = U;
    type Metric = M;
    type Parity = S;
}
