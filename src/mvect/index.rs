use crate::{
    basis::Basis,
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
    utils::contains::{IdxOf, IndexOf},
};
use generic_array::ArrayLength;
use typenum::{Len, Unsigned, B0};

impl<U: Unsigned, A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field>
    core::ops::Index<Basis<U, M, B0>> for Mvect<A, M, F>
where
    A: IndexOf<U>,
{
    type Output = F;
    #[inline(always)]
    fn index(&self, _: Basis<U, M, B0>) -> &Self::Output {
        let i = IdxOf::<A, U>::USIZE;
        &self.0[i]
    }
}

impl<U: Unsigned, A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field>
    core::ops::IndexMut<Basis<U, M, B0>> for Mvect<A, M, F>
where
    A: IndexOf<U>,
{
    #[inline(always)]
    fn index_mut(&mut self, _: Basis<U, M, B0>) -> &mut Self::Output {
        &mut self.0[IdxOf::<A, U>::USIZE]
    }
}
