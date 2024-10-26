use crate::{
    basis::{Basis, ZeroVect},
    metric::Metric,
};
use typenum::{Bit, Unsigned};

// -------------------------------------------------------------------------------------
// Zero cases
impl core::ops::Add<ZeroVect> for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn add(self, _: ZeroVect) -> Self::Output {
        self
    }
}
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Add<ZeroVect> for Basis<U, M, S> {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn add(self, _: ZeroVect) -> Self::Output {
        self
    }
}
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Add<Basis<U, M, S>> for ZeroVect {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn add(self, rhs: Basis<U, M, S>) -> Self::Output {
        rhs
    }
}
