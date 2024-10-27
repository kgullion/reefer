#![allow(non_upper_case_globals)]
use crate::{basis::Basis, ta};
use typenum::{B0, P1, U0, U1, U2, U3, U4, U5, U6, U7, Z0};

// 2D Projective Geometric Algebra
pub type Metric = ta![Z0, P1, P1];
pub type Pga2d<U> = Basis<U, Metric, B0>;

pub const scalar: Pga2d<U0> = Pga2d::<U0>::new();
pub const e0: Pga2d<U1> = Pga2d::<U1>::new();
pub const e1: Pga2d<U2> = Pga2d::<U2>::new();
pub const e01: Pga2d<U3> = Pga2d::<U3>::new();
pub const e2: Pga2d<U4> = Pga2d::<U4>::new();
pub const e02: Pga2d<U5> = Pga2d::<U5>::new();
pub const e12: Pga2d<U6> = Pga2d::<U6>::new();
pub const e012: Pga2d<U7> = Pga2d::<U7>::new();
