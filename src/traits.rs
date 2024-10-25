pub trait GeometricProduct<Rhs> {
    type Output;
    fn geo_prod(self, rhs: Rhs) -> Self::Output;
}
pub trait Dual {
    type Output;
    fn dual(self) -> Self::Output;
}
pub trait Undual {
    type Output;
    fn undual(self) -> Self::Output;
}
