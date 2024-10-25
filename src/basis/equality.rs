use super::{Basis, ZeroVector};
use crate::metric::Metric;
use core::ops::BitAnd;
use typenum::{And, Bit, Eq, IsEqual, Unsigned, B0, B1};

// 0==0
impl IsEqual<ZeroVector> for ZeroVector {
    type Output = B1;
    fn is_equal(self, _: ZeroVector) -> Self::Output {
        Self::Output::default()
    }
}
// 0!=B
impl<U: Unsigned, M: Metric, S: Bit> IsEqual<Basis<U, M, S>> for ZeroVector {
    type Output = B0;
    fn is_equal(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// B!=0
impl<U: Unsigned, M: Metric, S: Bit> IsEqual<ZeroVector> for Basis<U, M, S> {
    type Output = B0;
    fn is_equal(self, _: ZeroVector) -> Self::Output {
        Self::Output::default()
    }
}
// B==B <=> LU==RU && LS==RS
impl<
        LU: Unsigned + IsEqual<RU, Output: BitAnd<Eq<LS, RS>, Output: Bit>>,
        RU: Unsigned,
        M: Metric,
        LS: Bit + IsEqual<RS>,
        RS: Bit,
    > IsEqual<Basis<RU, M, RS>> for Basis<LU, M, LS>
where
    Eq<LU, RU>: BitAnd<Eq<LS, RS>>,
{
    type Output = And<Eq<LU, RU>, Eq<LS, RS>>;
    fn is_equal(self, _: Basis<RU, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// Eq & PartialEq
impl core::cmp::Eq for ZeroVector {}
impl<U: Unsigned, M: Metric, S: Bit> core::cmp::Eq for Basis<U, M, S> where Self: IsEqual<Self> {}

impl<R> PartialEq<R> for ZeroVector
where
    Self: IsEqual<R>,
{
    #[inline(always)]
    fn eq(&self, _: &R) -> bool {
        <Self as IsEqual<R>>::Output::BOOL
    }
}
impl<U: Unsigned, M: Metric, S: Bit, R> PartialEq<R> for Basis<U, M, S>
where
    Self: IsEqual<R>,
{
    #[inline(always)]
    fn eq(&self, _: &R) -> bool {
        <Self as IsEqual<R>>::Output::BOOL
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use typenum::{assert_type_eq, tarr, U0, U1, U10, U2, U3, U4, U5, U6, U7, U8, U9};
    #[test]
    fn test_is_equal() {
        assert_type_eq!(ZeroVector, ZeroVector);
        assert_type_eq!(Basis<U0, tarr![], B0>, Basis<U0, tarr![], B0>);
        // TODO: Add more tests
    }
}
