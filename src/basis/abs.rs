impl Abs for ZeroVector {
    type Output = ZeroVector;
}
impl<U: Unsigned, M: Metric, S: Bit> Abs for Basis<U, M, S>
where
    Basis<U, M, S>: BasisInfo,
    Basis<U, M, B0>: BasisInfo,
{
    type Output = Basis<U, M, B0>;
}
