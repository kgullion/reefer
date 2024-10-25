use crate::{
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
};
use core::marker::PhantomData;
use generic_array::{ArrayLength, GenericArray};
use typenum::{Len, Length};

// --------------------------------------------
// Default - create a new multivector with all elements set to zero
impl<BS: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field> core::default::Default
    for Mvect<BS, M, F>
{
    /// Create a new multivector from a GenericArray of field elements.
    #[inline(always)]
    fn default() -> Self {
        Mvect(GenericArray::<F, Length<BS>>::default(), PhantomData)
    }
}
