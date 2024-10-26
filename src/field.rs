use crate::{
    basis::Basis,
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
};
use generic_array::ArrayLength;
use num_traits::{NumAssignOps, Signed};
use typenum::{Bit, Len, Prod, Unsigned};

pub trait Field: Signed + NumAssignOps + PartialOrd + Default + Clone + Sized {}
impl Field for f32 {} // TODO: implement for i32, f64, etc.

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

impl<A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric> core::ops::Mul<Mvect<A, M, f32>> for f32
where
    Mvect<A, M, f32>: core::ops::Mul<f32>,
{
    type Output = Prod<Mvect<A, M, f32>, f32>;
    fn mul(self, rhs: Mvect<A, M, f32>) -> Self::Output {
        rhs * self
    }
}
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Mul<Basis<U, M, S>> for f32
where
    Basis<U, M, S>: core::ops::Mul<f32>,
{
    type Output = Prod<Basis<U, M, S>, f32>;
    fn mul(self, rhs: Basis<U, M, S>) -> Self::Output {
        rhs * self
    }
}
