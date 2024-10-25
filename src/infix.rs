use crate::{
    basis::{Basis, ZeroVector},
    field::Field,
    metric::Metric,
    mvect::{BasisSet, Mvect},
    traits::GeometricProduct,
};
use generic_array::ArrayLength;
use typenum::{Bit, Len, Unsigned};

// Implementations of the infix operators for Geometric Algebra.

impl<R> core::ops::Mul<R> for ZeroVector
where
    Self: GeometricProduct<R>,
{
    type Output = <Self as GeometricProduct<R>>::Output;
    #[inline(always)]
    fn mul(self, rhs: R) -> Self::Output {
        self.geo_prod(rhs)
    }
}
impl<U: Unsigned, M: Metric, S: Bit, R> core::ops::Mul<R> for Basis<U, M, S>
where
    Self: GeometricProduct<R>,
{
    type Output = <Self as GeometricProduct<R>>::Output;
    #[inline(always)]
    fn mul(self, rhs: R) -> Self::Output {
        self.geo_prod(rhs)
    }
}
impl<A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field, R> core::ops::Mul<R>
    for Mvect<A, M, F>
where
    Self: GeometricProduct<R>,
{
    type Output = <Self as GeometricProduct<R>>::Output;
    #[inline(always)]
    fn mul(self, rhs: R) -> Self::Output {
        self.geo_prod(rhs)
    }
}
