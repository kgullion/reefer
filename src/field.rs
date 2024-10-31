use crate::{
    basis::{Basis, ZeroVect},
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
    ta,
};
use core::ops::{Mul, Neg};
use generic_array::ArrayLength;
use num_traits::{NumAssignOps, Signed};
use typenum::{Bit, Len, Prod, Unsigned};
pub trait Field: Signed + Neg + NumAssignOps + PartialOrd + Default + Clone + Sized {}

// /// handy when a faster way to negate is available
// pub trait NegIf: Neg<Output = Self> + Clone {
//     #[inline]
//     fn neg_if(&mut self, cond: bool) {
//         if cond {
//             *self = -self.clone();
//         }
//     }
// }
// impl NegIf for f32 {
//     #[inline]
//     fn neg_if(&mut self, dagger: bool) {
//         // avoid branching
//         let cloak = unsafe { core::mem::transmute::<&mut f32, &mut u32>(self) };
//         *cloak ^= (dagger as u32) << 31;
//     }
// }

macro_rules! build_field_impls {
    ($field:ident) => {
        impl Field for $field {}

        impl<A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric> Mul<Mvect<A, M, $field>>
            for $field
        where
            Mvect<A, M, $field>: Mul<$field>,
        {
            type Output = Prod<Mvect<A, M, $field>, $field>;
            #[inline(always)]
            fn mul(self, rhs: Mvect<A, M, $field>) -> Self::Output {
                rhs * self
            }
        }

        impl<U: Unsigned, M: Metric, S: Bit> Mul<Basis<U, M, S>> for $field
        where
            Basis<U, M, S>: Mul<$field>,
        {
            type Output = Prod<Basis<U, M, S>, $field>;
            #[inline(always)]
            fn mul(self, rhs: Basis<U, M, S>) -> Self::Output {
                rhs * self
            }
        }

        impl<M: Metric> Mul<ZeroVect<M>> for $field {
            type Output = Mvect<ta![], M, $field>;
            #[inline(always)]
            fn mul(self, _: ZeroVect<M>) -> Self::Output {
                Self::Output::default()
            }
        }
    };
}

build_field_impls!(f32);
