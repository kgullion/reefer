use crate::GeometricObject;
use typenum::{TypeArray, Unsigned};

pub trait Commutator<Rhs> {
    type Output;
    fn commutator(self, rhs: Rhs) -> Self::Output;
}
pub trait ScalarProduct<Rhs> {
    type Output;
    fn scalar_prod(self, rhs: Rhs) -> Self::Output;
}
pub trait FatDot<Rhs> {
    type Output;
    fn fat_dot(self, rhs: Rhs) -> Self::Output;
}
pub trait Dual {
    type Output;
    fn dual(self) -> Self::Output;
}
pub trait Undual {
    type Output;
    fn undual(self) -> Self::Output;
}
pub trait Graded<G: Unsigned> {
    type BasisSet: TypeArray;
    fn graded(self) -> impl GeometricObject;
}
pub trait Involute {
    type Output;
    fn involute(self) -> Self::Output;
}
pub trait Reverse {
    type Output;
    fn reverse(self) -> Self::Output;
}
pub trait Conjugate {
    type Output;
    fn conjugate(self) -> Self::Output;
}
pub trait Inverse {
    type Output;
    fn inverse(self) -> Option<Self::Output>;
}
pub trait Normalize {
    fn normalize(self) -> Self;
}
pub trait Pow {
    type Output;
    type Power;
    fn pow(self, x: Self::Power) -> Self::Output;
}
pub trait Exp {
    type Output;
    fn exp(self) -> Self::Output;
}
pub trait Log {
    type Output;
    fn log(self) -> Self::Output; // TODO: also an Option?
}
pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Option<Self::Output>;
}
