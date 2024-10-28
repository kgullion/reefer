use crate::{
    collector::{CollectInto, Collector},
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
};
use generic_array::ArrayLength;
use typenum::Len;

// --------------------------------------------
// PartialEq - compare two multivectors
struct MvPartialEq;
// Collect the results of comparing two multivectors
impl<'a, F: Field> Collector<F, bool> for MvPartialEq {
    #[inline(always)]
    fn collect_both(out: bool, left: &F, right: &F) -> bool {
        out && left == right
    }
    #[inline(always)]
    fn collect_just_left(out: bool, left: &F) -> bool {
        out && left == &F::zero()
    }
    #[inline(always)]
    fn collect_just_right(out: bool, right: &F) -> bool {
        out && &F::zero() == right
    }
}
// PartialEq
impl<
        BS: BasisSet<M> + Len<Output: ArrayLength>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + CollectInto<F, MvPartialEq, bool, BS, RBS>,
    > core::cmp::PartialEq<Mvect<RBS, M, F>> for Mvect<BS, M, F>
{
    #[inline(always)]
    fn eq(&self, other: &Mvect<RBS, M, F>) -> bool {
        MvPartialEq::do_collect::<BS, RBS>(true, &self.0, &other.0)
    }
}
// Implement Total Eq for Fields that support it
impl<
        BS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: core::cmp::Eq + Field + CollectInto<F, MvPartialEq, bool, BS, BS>,
    > core::cmp::Eq for Mvect<BS, M, F>
{
}
