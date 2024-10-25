use crate::{
    basis::Basis,
    metric::Metric,
    utils::count::{Count, CountOf},
};
use typenum::{Bit, Unsigned, B1};

use super::ZeroVector;

pub trait Grade {
    /// Number of basis vectors in the basis.
    fn grade(self) -> usize;
}
impl<U: Unsigned + CountOf<typenum::B1>> Grade for U {
    #[inline(always)]
    fn grade(self) -> usize {
        Count::<U, B1>::to_usize()
    }
}
impl<U: Unsigned + CountOf<typenum::B1>, M: Metric, S: Bit> Grade for Basis<U, M, S> {
    #[inline(always)]
    fn grade(self) -> usize {
        Count::<U, B1>::to_usize()
    }
}
impl Grade for ZeroVector {
    #[inline(always)]
    fn grade(self) -> usize {
        0
    }
}
