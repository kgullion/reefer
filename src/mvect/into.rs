use typenum::{tarr, Bit, Unsigned, B0, B1};

use crate::{
    basis::{Basis, ZeroVector},
    field::Field,
    metric::Metric,
    mvect::Mvect,
};

// --------------------------------------------
// IntoBasisSet - convert a Basis or ZeroVector type into a BasisSet
pub trait IntoBasisSet {
    type Output;
}
impl IntoBasisSet for ZeroVector {
    type Output = tarr![];
}
impl<U: Unsigned, M: Metric, S: Bit> IntoBasisSet for Basis<U, M, S> {
    type Output = tarr![U];
}

// --------------------------------------------
// IntoMv - convert a Basis or ZeroVector type into a Mvect instance
pub trait IntoMv<F: Field> {
    type Output;
    fn into_mv() -> Self::Output;
}
impl<F: Field, U: Unsigned, M: Metric> IntoMv<F> for Basis<U, M, B0> {
    type Output = Mvect<tarr![U], M, F>;
    fn into_mv() -> Self::Output {
        let mut out = Mvect::<tarr![U], M, F>::default();
        out.0[0] = F::one();
        out
    }
}
impl<F: Field, U: Unsigned, M: Metric> IntoMv<F> for Basis<U, M, B1> {
    type Output = Mvect<tarr![U], M, F>;
    fn into_mv() -> Self::Output {
        let mut out = Mvect::<tarr![U], M, F>::default();
        out.0[0] = -F::one();
        out
    }
}