pub mod add;
pub mod basis_set;
pub mod default;
pub mod equality;
pub mod into;
pub mod mul;

use crate::{field::Field, metric::Metric, mvect::basis_set::BasisSet};
use core::marker::PhantomData;
use generic_array::{ArrayLength, GenericArray};
use typenum::{Len, Length, Unsigned};

/// multivector
#[derive(Clone)]
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
    use crate::basis::Basis;

    use super::*;
    use into::IntoMv;
    use typenum::{tarr, B0, P1, U0, U1, U2, U3, U4, U5, U6, U7, Z0};

    type Metric = tarr![Z0, P1, P1];
    type Pga2d<U> = Basis<U, Metric, B0>;

    type Scalar = Pga2d<U0>;
    type E0 = Pga2d<U1>;
    type E1 = Pga2d<U2>;
    type E01 = Pga2d<U3>;
    type E2 = Pga2d<U4>;
    type E02 = Pga2d<U5>;
    type E12 = Pga2d<U6>;
    type E012 = Pga2d<U7>;

    #[test]
    fn test_default() {
        type M = tarr![Z0, P1, P1];
        type BS = <tarr![U0, U1, U2, U4] as BasisSet<M>>::Output;
        let mv = Mvect::<BS, M, f32>::default();
        assert_eq!(mv.len(), 4);
        for &elem in mv.0.iter() {
            assert_eq!(elem, 0.0);
        }
    }

    #[test]
    fn test_into_mv() {
        let expected = Mvect::<tarr![U3], Metric, f32>::new(GenericArray::<f32, U1>::from([1.0]));
        let actual = <E01 as IntoMv<f32>>::into_mv();
        assert!(expected == actual);
    }

    #[test]
    fn test_eq() {
        let mv0 = <E01 as IntoMv<f32>>::into_mv();
        let mv1 = <E01 as IntoMv<f32>>::into_mv();
        assert!(mv0 == mv1);
        // TODO: test zeros vs not stored vals
    }

    #[test]
    fn test_add() {
        let e = <Scalar as IntoMv<f32>>::into_mv();
        let e0 = <E0 as IntoMv<f32>>::into_mv();

        let c = e + e0;
    }

    #[test]
    fn test_mul() {
        let e = <Scalar as IntoMv<f32>>::into_mv();
        let e1 = <E1 as IntoMv<f32>>::into_mv();
        println!("{:?}    {:?}", &e.0, &e1.0);
        let c = e1.clone() * (e + e1.clone());
        println!("{:?}", c.0);
    }
}
