use num_traits::{NumAssignOps, Signed};

pub trait Field: Signed + NumAssignOps + Default + Clone {}
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
