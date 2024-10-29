use crate::{
    basis::{Basis, ZeroVect},
    metric::Metric,
};
use typenum::{And, Bit, Eq, IsEqual, Unsigned, B0, B1};

// 0==0
impl<M: Metric> IsEqual<ZeroVect<M>> for ZeroVect<M> {
    type Output = B1;
    #[inline(always)]
    fn is_equal(self, _: ZeroVect<M>) -> Self::Output {
        Self::Output::default()
    }
}
// 0!=B
impl<U: Unsigned, M: Metric, S: Bit> IsEqual<Basis<U, M, S>> for ZeroVect<M> {
    type Output = B0;
    #[inline(always)]
    fn is_equal(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// B!=0
impl<U: Unsigned, M: Metric, S: Bit> IsEqual<ZeroVect<M>> for Basis<U, M, S> {
    type Output = B0;
    #[inline(always)]
    fn is_equal(self, _: ZeroVect<M>) -> Self::Output {
        Self::Output::default()
    }
}
// B==B <=> LU==RU && LS==RS
impl<
        LU: Unsigned + IsEqual<RU, Output: core::ops::BitAnd<Eq<LS, RS>, Output: Bit>>,
        RU: Unsigned,
        M: Metric,
        LS: Bit + IsEqual<RS>,
        RS: Bit,
    > IsEqual<Basis<RU, M, RS>> for Basis<LU, M, LS>
where
    Eq<LU, RU>: core::ops::BitAnd<Eq<LS, RS>>,
{
    type Output = And<Eq<LU, RU>, Eq<LS, RS>>;
    #[inline(always)]
    fn is_equal(self, _: Basis<RU, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// Eq & PartialEq
impl<M: Metric> core::cmp::Eq for ZeroVect<M> {}
impl<U: Unsigned, M: Metric, S: Bit> core::cmp::Eq for Basis<U, M, S> where Self: IsEqual<Self> {}

impl<R, M: Metric> PartialEq<R> for ZeroVect<M>
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
    use crate::ta;
    #[allow(unused_imports)]
    use typenum::{assert_type_eq, tarr, U0, U1, U10, U2, U3, U4, U5, U6, U7, U8, U9};
    #[test]
    fn test_is_equal() {
        assert_type_eq!(ZeroVect<ta![]>, ZeroVect<ta![]>);
        assert_type_eq!(Basis<U0, ta![], B0>, Basis<U0, ta![], B0>);
        // TODO: Add more tests
    }
}
