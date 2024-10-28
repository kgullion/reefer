use crate::{
    basis::{Basis, ZeroVect},
    metric::Metric,
};
use typenum::{Bit, Len, Unsigned, B0, B1, N1, P1, Z0};

/// Helper trait functionality for converting B0, B1, P1, N1, Z0 to the correct Basis
/// Mostly for comptime internal use

pub trait IntoBasis<U: Unsigned, M: Metric> {
    type Output;
}
impl<U: Unsigned + Len, M: Metric> IntoBasis<U, M> for B0
where
    Self: Into<Basis<U, M, B0>>,
{
    type Output = Basis<U, M, B0>;
}
impl<U: Unsigned + Len, M: Metric> IntoBasis<U, M> for B1
where
    Self: Into<Basis<U, M, B1>>,
{
    type Output = Basis<U, M, B1>;
}
impl<U: Unsigned + Len, M: Metric> IntoBasis<U, M> for Z0
where
    Self: Into<ZeroVect>,
{
    type Output = ZeroVect;
}
impl<U: Unsigned + Len, M: Metric> IntoBasis<U, M> for P1
where
    Self: Into<Basis<U, M, B0>>,
{
    type Output = Basis<U, M, B0>;
}
impl<U: Unsigned + Len, M: Metric> IntoBasis<U, M> for N1
where
    Self: Into<Basis<U, M, B1>>,
{
    type Output = Basis<U, M, B1>;
}

// only valid casts are defined
impl<U: Unsigned, M: Metric, S: Bit> From<S> for Basis<U, M, S> {
    // Bit is the Parity of the Sign
    #[inline(always)]
    fn from(_: S) -> Self {
        Self::default()
    }
}
impl From<Z0> for ZeroVect {
    // Zero is Zero
    #[inline(always)]
    fn from(_: Z0) -> Self {
        Self::default()
    }
}
impl<U: Unsigned, M: Metric> From<P1> for Basis<U, M, B0> {
    // +1 -> parity==false
    #[inline(always)]
    fn from(_: P1) -> Self {
        Self::default()
    }
}
impl<U: Unsigned, M: Metric> From<N1> for Basis<U, M, B1> {
    // -1 -> parity==true
    #[inline(always)]
    fn from(_: N1) -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ta;
    #[allow(unused_imports)]
    use typenum::{assert_type_eq, tarr, U0, U1, U2};
    #[test]
    fn into_basis() {
        #[allow(unused)]
        type M = ta![Z0, P1, P1];
        assert_type_eq!(Basis<U2, M, B0>, <B0 as IntoBasis<U2, M>>::Output);
        assert_type_eq!(Basis<U2, M, B1>, <B1 as IntoBasis<U2, M>>::Output);
        assert_type_eq!(Basis<U2, M, B0>, <P1 as IntoBasis<U2, M>>::Output);
        assert_type_eq!(Basis<U2, M, B1>, <N1 as IntoBasis<U2, M>>::Output);
        assert_type_eq!(ZeroVect, <Z0 as IntoBasis<U2, M>>::Output);
    }
}
