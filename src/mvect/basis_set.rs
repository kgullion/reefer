use crate::metric::Metric;
use core::ops::Add;
use generic_array::ArrayLength;
use typenum::{ATerm, Len, TArr, TypeArray, Unsigned, B1};

/// BasisSet stores the Bitmask of the Basis elements that are present in the multivector.
/// Together with the metric, this is enough to recover each Basis.
///
/// it also helps the compiler to not run rampant on every TypeArray it finds.
pub trait BasisSet<M: Metric>: TypeArray {
    type Output;
}
impl<M: Metric> BasisSet<M> for ATerm {
    type Output = ATerm;
}
impl<BS: BasisSet<M> + Len<Output: Unsigned + ArrayLength + Add<B1>>, U: Unsigned, M: Metric>
    BasisSet<M> for TArr<U, BS>
{
    type Output = TArr<U, BS>;
}
