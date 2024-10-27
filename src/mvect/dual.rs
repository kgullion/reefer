use super::{basis_set::BasisSet, Mvect};
use crate::{
    basis::{Basis, BasisInfo, ZeroVect},
    field::Field,
    metric::Metric,
    ta,
    traits::{Dual, Undual},
    utils::{
        contains::{IdxOf, IndexOf},
        typeset::{Union, UnionMerge},
    },
};
use core::ops;
use generic_array::ArrayLength;
use typenum::{bit::B0, Bit, Len, TypeArray, Unsigned, Xor};

impl<
        A: BasisSet<M>
            + Len<Output: ArrayLength>
            + DualBasisType<M, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + DualBasisRun<<A as DualBasisType<M>>::Output, M, F>,
        M: Metric,
        F: Field,
    > Dual for Mvect<A, M, F>
{
    type Output = Mvect<<A as DualBasisType<M>>::Output, M, F>;
    fn dual(self) -> Self::Output {
        // TODO: cycles can be built at comp time, letting us just swap indices around instead of
        // building a new array, but I cba rn plus *maybe* the compiler will do it for us anyway
        let mut out = Self::Output::default();
        <A as DualBasisRun<<A as DualBasisType<M>>::Output, M, F>>::dualize(&mut out.0, &self.0);
        out
    }
}
impl<
        A: BasisSet<M>
            + Len<Output: ArrayLength>
            + UndualBasisType<M, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + UndualBasisRun<<A as UndualBasisType<M>>::Output, M, F>,
        M: Metric,
        F: Field,
    > Undual for Mvect<A, M, F>
{
    type Output = Mvect<<A as UndualBasisType<M>>::Output, M, F>;
    fn undual(self) -> Self::Output {
        // TODO: cycles can be built at comp time, letting us just swap indices around instead of
        // building a new array, but I cba rn plus *maybe* the compiler will do it for us anyway
        let mut out = Self::Output::default();
        <A as UndualBasisRun<<A as UndualBasisType<M>>::Output, M, F>>::undualize(
            &mut out.0, &self.0,
        );
        out
    }
}
// ----
pub trait DualBasisType<M: Metric>: TypeArray {
    type Output: TypeArray;
}
impl<M: Metric> DualBasisType<M> for ta![] {
    type Output = ta![];
}
impl<
        U: Unsigned + ops::BitXor<M::Psuedoscalar>,
        A: BasisSet<M> + DualBasisType<M, Output: UnionMerge<ta![Xor<U, M::Psuedoscalar>]>>,
        M: Metric,
    > DualBasisType<M> for ta![U | A]
{
    type Output = Union<<A as DualBasisType<M>>::Output, ta![Xor<U, M::Psuedoscalar>]>;
}
pub trait DualBasisRun<OUT: BasisSet<M>, M: Metric, F: Field>: BasisSet<M> {
    type Dual;
    fn dualize(out: &mut [F], data: &[F]);
}
impl<M: Metric, F: Field> DualBasisRun<ta![], M, F> for ta![] {
    type Dual = ZeroVect;
    fn dualize(_out: &mut [F], _data: &[F]) {}
}
impl<
        OUT: BasisSet<M> + IndexOf<<<Basis<U, M, B0> as Dual>::Output as BasisInfo>::Mask>,
        U: Unsigned + ops::BitXor<M::Psuedoscalar>,
        A: DualBasisRun<OUT, M, F>,
        M: Metric,
        F: Field,
    > DualBasisRun<OUT, M, F> for ta![U | A]
where
    Basis<U, M, B0>: Dual<Output: BasisInfo<Parity: Bit>>,
    ta![U | A]: BasisSet<M>,
{
    type Dual = <Basis<U, M, B0> as Dual>::Output;
    fn dualize(out: &mut [F], data: &[F]) {
        // get the index of the dual basis in the output array and set its value
        let i = IdxOf::<OUT, <Self::Dual as BasisInfo>::Mask>::USIZE;
        if <Self::Dual as BasisInfo>::Parity::BOOL {
            out[i] = -data[0].clone();
        } else {
            out[i] = data[0].clone();
        }
        A::dualize(&mut out[1..], &data[1..]);
    }
}
// ----
pub trait UndualBasisType<M: Metric>: TypeArray {
    type Output: TypeArray;
}
impl<M: Metric> UndualBasisType<M> for ta![] {
    type Output = ta![];
}
impl<
        U: Unsigned + ops::BitXor<M::Psuedoscalar>,
        A: BasisSet<M> + UndualBasisType<M, Output: UnionMerge<ta![Xor<U, M::Psuedoscalar>]>>,
        M: Metric,
    > UndualBasisType<M> for ta![U | A]
{
    type Output = Union<<A as UndualBasisType<M>>::Output, ta![Xor<U, M::Psuedoscalar>]>;
}
trait UndualBasisRun<OUT: BasisSet<M>, M: Metric, F: Field>: BasisSet<M> {
    type Undual;
    fn undualize(out: &mut [F], data: &[F]);
}
impl<M: Metric, F: Field> UndualBasisRun<ta![], M, F> for ta![] {
    type Undual = ZeroVect;
    fn undualize(_out: &mut [F], _data: &[F]) {}
}
impl<
        OUT: BasisSet<M> + IndexOf<<<Basis<U, M, B0> as Undual>::Output as BasisInfo>::Mask>,
        U: Unsigned + ops::BitXor<M::Psuedoscalar>,
        A: UndualBasisRun<OUT, M, F>,
        M: Metric,
        F: Field,
    > UndualBasisRun<OUT, M, F> for ta![U | A]
where
    Basis<U, M, B0>: Undual<Output: BasisInfo<Parity: Bit>>,
    ta![U | A]: BasisSet<M>,
{
    type Undual = <Basis<U, M, B0> as Undual>::Output;
    fn undualize(out: &mut [F], data: &[F]) {
        // get the index of the dual basis in the output array and set its value
        let i = IdxOf::<OUT, <Self::Undual as BasisInfo>::Mask>::USIZE;
        if <Self::Undual as BasisInfo>::Parity::BOOL {
            out[i] = -data[0].clone();
        } else {
            out[i] = data[0].clone();
        }
        A::undualize(&mut out[1..], &data[1..]);
    }
}
