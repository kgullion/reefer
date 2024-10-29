use num_traits::One;
use typenum::{Unsigned, B0, B1};

use crate::{
    basis::{Basis, ZeroVect},
    field::Field,
    metric::Metric,
    ta,
};

use super::Mvect;

impl<M: Metric, F: Field> From<ZeroVect<M>> for Mvect<ta![], M, F> {
    #[inline(always)]
    fn from(_: ZeroVect<M>) -> Self {
        Self::default()
    }
}
impl<U: Unsigned, M: Metric, F: Field + One> From<Basis<U, M, B0>> for Mvect<ta![U], M, F> {
    #[inline(always)]
    fn from(_: Basis<U, M, B0>) -> Self {
        let mut out = Self::default();
        out.0[0] = F::one();
        out
    }
}
impl<U: Unsigned, M: Metric, F: Field + One + core::ops::Neg> From<Basis<U, M, B1>>
    for Mvect<ta![U], M, F>
{
    #[inline(always)]
    fn from(_: Basis<U, M, B1>) -> Self {
        let mut out = Self::default();
        out.0[0] = -F::one();
        out
    }
}
