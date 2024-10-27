use crate::{
    basis::{Basis, ZeroVect},
    metric::Metric,
    mvect::into::IntoBasisSet,
    ta,
    traits::Graded,
    utils::{
        count::{Count, CountOf},
        Branch, If,
    },
    GeometricObject,
};
use typenum::{Bit, Eq, IsEqual, Unsigned, B1};

impl<G: Unsigned> Graded<G> for ZeroVect {
    type BasisSet = ta![];
    fn grade(self) -> usize {
        0
    }
    #[allow(refining_impl_trait)]
    #[inline(always)]
    fn graded(self) -> ZeroVect {
        self
    }
}
impl<G: Unsigned, U: Unsigned, M: Metric, S: Bit> Graded<G> for Basis<U, M, S>
where
    U: CountOf<
        B1,
        Count: IsEqual<
            G,
            Output: Branch<Basis<U, M, S>, ZeroVect, Output: GeometricObject + IntoBasisSet>,
        >,
    >,
{
    type BasisSet = <If<Eq<Count<U, B1>, G>, Basis<U, M, S>, ZeroVect> as IntoBasisSet>::Output;
    fn grade(self) -> usize {
        Count::<U, B1>::USIZE
    }
    #[allow(refining_impl_trait)]
    #[inline(always)]
    fn graded(self) -> If<Eq<Count<U, B1>, G>, Basis<U, M, S>, ZeroVect> {
        If::<Eq<Count<U, B1>, G>, Basis<U, M, S>, ZeroVect>::default()
    }
}
