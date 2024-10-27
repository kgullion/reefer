pub mod add;
pub mod default;
pub mod display;
pub mod dual;
pub mod equality;
pub mod grade;
pub mod into;
pub mod mul;
pub mod negations;

use crate::{metric::Metric, ta};
use core::marker::PhantomData;
use core::ops;
use typenum::{
    Bit, Cmp, Compare, Eq, Equal, Greater, Integer, IsEqual, Less, Ord, Prod, TypeArray, Unsigned,
    Xor, B0, B1, N1, P1, U0, Z0,
};

// -------------------------------------
/// A Basis is a signed product of unit length basis vectors in a geometric algebra.
#[derive(Clone, Copy, Debug)]
pub struct Basis<U: Unsigned, M: Metric, S: Bit>(PhantomData<(U, M, S)>);

// the zero vector is a special case, every other basis is length 1
/// Represents the zero vector. You probably don't need to use this directly and got it by multiplying
/// two degenerate Basis elements. Mostly here for the compiler.
pub struct ZeroVect;

// -------------------------------------
// basis operations
// const new
impl<U: Unsigned, M: Metric, S: Bit> Basis<U, M, S> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
