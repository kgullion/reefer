#![allow(non_upper_case_globals)]
use crate::{basis::Basis, ta};
use typenum::{
    B0, P1, U0, U1, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U2, U20, U21, U22, U23, U24,
    U25, U26, U27, U28, U29, U3, U30, U31, U32, U33, U34, U35, U36, U37, U38, U39, U4, U40, U41,
    U42, U43, U44, U45, U46, U47, U48, U49, U5, U50, U51, U52, U53, U54, U55, U56, U57, U58, U59,
    U6, U60, U61, U62, U63, U7, U8, U9,
};

// 6D Vector Geometric Algebra
pub type Metric = ta![P1, P1, P1, P1, P1, P1];
type Vga6d<U> = Basis<U, Metric, B0>;

pub const e: Vga6d<U0> = Vga6d::<U0>::new();
pub const e1: Vga6d<U1> = Vga6d::<U1>::new();
pub const e2: Vga6d<U2> = Vga6d::<U2>::new();
pub const e12: Vga6d<U3> = Vga6d::<U3>::new();
pub const e3: Vga6d<U4> = Vga6d::<U4>::new();
pub const e13: Vga6d<U5> = Vga6d::<U5>::new();
pub const e23: Vga6d<U6> = Vga6d::<U6>::new();
pub const e123: Vga6d<U7> = Vga6d::<U7>::new();
pub const e4: Vga6d<U8> = Vga6d::<U8>::new();
pub const e14: Vga6d<U9> = Vga6d::<U9>::new();
pub const e24: Vga6d<U10> = Vga6d::<U10>::new();
pub const e124: Vga6d<U11> = Vga6d::<U11>::new();
pub const e34: Vga6d<U12> = Vga6d::<U12>::new();
pub const e134: Vga6d<U13> = Vga6d::<U13>::new();
pub const e234: Vga6d<U14> = Vga6d::<U14>::new();
pub const e1234: Vga6d<U15> = Vga6d::<U15>::new();
pub const e5: Vga6d<U16> = Vga6d::<U16>::new();
pub const e15: Vga6d<U17> = Vga6d::<U17>::new();
pub const e25: Vga6d<U18> = Vga6d::<U18>::new();
pub const e125: Vga6d<U19> = Vga6d::<U19>::new();
pub const e35: Vga6d<U20> = Vga6d::<U20>::new();
pub const e135: Vga6d<U21> = Vga6d::<U21>::new();
pub const e235: Vga6d<U22> = Vga6d::<U22>::new();
pub const e1235: Vga6d<U23> = Vga6d::<U23>::new();
pub const e45: Vga6d<U24> = Vga6d::<U24>::new();
pub const e145: Vga6d<U25> = Vga6d::<U25>::new();
pub const e245: Vga6d<U26> = Vga6d::<U26>::new();
pub const e1245: Vga6d<U27> = Vga6d::<U27>::new();
pub const e345: Vga6d<U28> = Vga6d::<U28>::new();
pub const e1345: Vga6d<U29> = Vga6d::<U29>::new();
pub const e2345: Vga6d<U30> = Vga6d::<U30>::new();
pub const e12345: Vga6d<U31> = Vga6d::<U31>::new();
pub const e6: Vga6d<U32> = Vga6d::<U32>::new();
pub const e16: Vga6d<U33> = Vga6d::<U33>::new();
pub const e26: Vga6d<U34> = Vga6d::<U34>::new();
pub const e126: Vga6d<U35> = Vga6d::<U35>::new();
pub const e36: Vga6d<U36> = Vga6d::<U36>::new();
pub const e136: Vga6d<U37> = Vga6d::<U37>::new();
pub const e236: Vga6d<U38> = Vga6d::<U38>::new();
pub const e1236: Vga6d<U39> = Vga6d::<U39>::new();
pub const e46: Vga6d<U40> = Vga6d::<U40>::new();
pub const e146: Vga6d<U41> = Vga6d::<U41>::new();
pub const e246: Vga6d<U42> = Vga6d::<U42>::new();
pub const e1246: Vga6d<U43> = Vga6d::<U43>::new();
pub const e346: Vga6d<U44> = Vga6d::<U44>::new();
pub const e1346: Vga6d<U45> = Vga6d::<U45>::new();
pub const e2346: Vga6d<U46> = Vga6d::<U46>::new();
pub const e12346: Vga6d<U47> = Vga6d::<U47>::new();
pub const e56: Vga6d<U48> = Vga6d::<U48>::new();
pub const e156: Vga6d<U49> = Vga6d::<U49>::new();
pub const e256: Vga6d<U50> = Vga6d::<U50>::new();
pub const e1256: Vga6d<U51> = Vga6d::<U51>::new();
pub const e356: Vga6d<U52> = Vga6d::<U52>::new();
pub const e1356: Vga6d<U53> = Vga6d::<U53>::new();
pub const e2356: Vga6d<U54> = Vga6d::<U54>::new();
pub const e12356: Vga6d<U55> = Vga6d::<U55>::new();
pub const e456: Vga6d<U56> = Vga6d::<U56>::new();
pub const e1456: Vga6d<U57> = Vga6d::<U57>::new();
pub const e2456: Vga6d<U58> = Vga6d::<U58>::new();
pub const e12456: Vga6d<U59> = Vga6d::<U59>::new();
pub const e3456: Vga6d<U60> = Vga6d::<U60>::new();
pub const e13456: Vga6d<U61> = Vga6d::<U61>::new();
pub const e23456: Vga6d<U62> = Vga6d::<U62>::new();
pub const e123456: Vga6d<U63> = Vga6d::<U63>::new();
