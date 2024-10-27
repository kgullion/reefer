#![allow(non_upper_case_globals)]
use crate::{basis::Basis, ta};
use typenum::{B0, P1, U0, U1, U2, U3, U4, U5, U6, U7};

// 2D Projective Geometric Algebra
pub type Metric = ta![P1, P1, P1];
type Vga3d<U> = Basis<U, Metric, B0>;

pub const scalar: Vga3d<U0> = Vga3d::<U0>::new();
pub const x: Vga3d<U1> = Vga3d::<U1>::new();
pub const y: Vga3d<U2> = Vga3d::<U2>::new();
pub const xy: Vga3d<U3> = Vga3d::<U3>::new();
pub const z: Vga3d<U4> = Vga3d::<U4>::new();
pub const xz: Vga3d<U5> = Vga3d::<U5>::new();
pub const yz: Vga3d<U6> = Vga3d::<U6>::new();
pub const xyz: Vga3d<U7> = Vga3d::<U7>::new();
