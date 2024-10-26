pub mod add;
pub mod default;
pub mod dual;
pub mod equality;
pub mod grade;
pub mod into;
pub mod mul;
pub mod negations;

use crate::metric::Metric;
use core::marker::PhantomData;
use typenum::{Bit, Unsigned, B0, B1};

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

use core::fmt;
impl fmt::Display for ZeroVect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0")
    }
}

impl<U: Unsigned, M: Metric> fmt::Display for Basis<U, M, B0> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "e")?;
        let mut n = U::to_usize();
        let mut i = 0;
        while n > 0 {
            if n & 1 == 1 {
                write!(f, "{}", i)?;
            }
            n >>= 1;
            i += 1;
        }
        Ok(())
    }
}
impl<U: Unsigned, M: Metric> fmt::Display for Basis<U, M, B1> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-{}", Basis::<U, M, B0>::new())
    }
}
