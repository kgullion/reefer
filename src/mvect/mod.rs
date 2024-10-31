pub mod add;
pub mod basis_set;
pub mod default;
pub mod display;
pub mod dual;
pub mod equality;
pub mod grade;
pub mod index;
pub mod into;
pub mod mul;
pub mod neg;

use crate::{field::Field, metric::Metric, mvect::basis_set::BasisSet};
use core::marker::PhantomData;
use generic_array::{ArrayLength, GenericArray};
use typenum::{Len, Length, Unsigned};

/// multivector
#[derive(Clone, Debug)]
pub struct Mvect<BS: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field>(
    GenericArray<F, Length<BS>>,
    PhantomData<M>,
);
impl<A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field + Copy> Copy for Mvect<A, M, F> where
    GenericArray<F, Length<A>>: Copy
{
}

impl<BS: BasisSet<M> + Len<Output: Unsigned + ArrayLength>, M: Metric, F: Field> Mvect<BS, M, F> {
    /// Create a new multivector from a GenericArray of field elements.
    #[inline(always)]
    pub fn new(data: GenericArray<F, Length<BS>>) -> Self {
        Mvect(data, PhantomData)
    }
    /// Length of the multivector
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use crate::{
        basis::Basis,
        pga2d::{e0, e01, e012, e02, e1, e12, e2, scalar as e, Metric},
        ta,
    };
    use typenum::{tarr, B0, P1, U0, U1, U2, U3, U4, U5, U6, U7, Z0};

    #[test]
    fn test_default() {
        // type M = ta![Z0, P1, P1];
        // type BS = <ta![U0, U1, U2, U4] as BasisSet<M>>::Output;
        // let mv = Mvect::<BS, M, f32>::default();
        // assert_eq!(mv.len(), 4);
        // assert_eq!(core::mem::size_of_val(&mv), 4 * core::mem::size_of::<f32>()); // !!
        // for &elem in mv.0.iter() {
        //     assert_eq!(elem, 0.0);
        // }
    }

    #[test]
    fn test_into_mv() {
        // let expected = Mvect::<ta![U3], Metric, f32>::new(GenericArray::<f32, U1>::from([1.0]));
        // let actual = 1.0 * e01;
        // assert!(expected == actual);
    }

    #[test]
    fn test_eq() {
        let mv0 = 1.0 * e01;
        let mv1 = 1.0 * e01 + 1.0 * e012;
        assert!(mv0 == mv1 - 1.0 * e012);
    }
}
