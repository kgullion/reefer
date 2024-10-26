use crate::{metric::Metric, ta};
use core::ops::Add;
use generic_array::ArrayLength;
use typenum::{Len, TypeArray, Unsigned, B1};

/// BasisSet stores the Bitmask of the Basis elements that are present in the multivector.
/// Together with the metric, this is enough to recover each Basis.
///
/// it also helps the compiler to not run rampant on every TypeArray it finds.
pub trait BasisSet<M: Metric>: TypeArray + Copy + Clone {
    type Output;
}
impl<M: Metric> BasisSet<M> for ta![] {
    type Output = ta![];
}
impl<BS: BasisSet<M> + Len<Output: Unsigned + ArrayLength + Add<B1>>, U: Unsigned, M: Metric>
    BasisSet<M> for ta![U | BS]
{
    type Output = ta![U | BS];
}
