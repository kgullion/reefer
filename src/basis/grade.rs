use crate::{
    basis::{Basis, ZeroVect},
    metric::Metric,
    traits::Grade,
    utils::{
        count::{Count, CountOf},
        Branch, If,
    },
};
use typenum::{Bit, Eq, IsEqual, Unsigned, B1};

// -------------------------------------------------------------------------------------
impl<M: Metric> Grade for ZeroVect<M> {
    #[inline(always)]
    fn grade(self) -> usize {
        0
    }
}
impl<U: Unsigned + CountOf<B1>, M: Metric, S: Bit> Grade for Basis<U, M, S> {
    #[inline(always)]
    fn grade(self) -> usize {
        Count::<U, B1>::USIZE
    }
}

// -------------------------------------------------------------------------------------
impl<G: Unsigned, M: Metric> core::ops::Rem<G> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn rem(self, _: G) -> Self::Output {
        self
    }
}
impl<
        G: Unsigned + IsEqual<U::Count, Output: Branch<Basis<U, M, S>, ZeroVect<M>, Output: Default>>,
        U: Unsigned + CountOf<B1>,
        M: Metric,
        S: Bit,
    > core::ops::Rem<G> for Basis<U, M, S>
{
    type Output = If<Eq<G, U::Count>, Self, ZeroVect<M>>;
    #[inline(always)]
    fn rem(self, _: G) -> Self::Output {
        Self::Output::default()
    }
}
