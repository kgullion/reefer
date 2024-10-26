use crate::{
    basis::Basis,
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
    ta,
};
use core::fmt;
use generic_array::ArrayLength;
use typenum::{Bit, Len, TArr, UInt, Unsigned, B0, U0};

// Display for Mvect
impl<
        A: MvDisplayHead<M, F> + BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + fmt::Display,
    > fmt::Display for Mvect<A, M, F>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <A as MvDisplayHead<M, F>>::fmt(&self.0, f)
    }
}
// Display first element of Mvect
trait MvDisplayHead<M: Metric, F: Field> {
    fn fmt(data: &[F], f: &mut fmt::Formatter) -> fmt::Result;
}
impl<M: Metric, F: Field + fmt::Display> MvDisplayHead<M, F> for ta![] {
    #[inline(always)]
    fn fmt(_data: &[F], f: &mut fmt::Formatter) -> fmt::Result {
        // no head found, must be 0
        write!(f, "{}", F::zero())
    }
}
impl<A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field + fmt::Display>
    MvDisplayHead<M, F> for ta![U0 | A]
where
    ta![U0 | A]: BasisSet<M>,
    A: MvDisplayHead<M, F> + MvDisplayTail<M, F>,
{
    #[inline(always)]
    fn fmt(data: &[F], f: &mut fmt::Formatter) -> fmt::Result {
        if data[0] == F::zero() {
            // keep looking for the first non-zero element
            <A as MvDisplayHead<M, F>>::fmt(&data[1..], f)
        } else {
            // U0 represents a scalar (with no basis vectors)
            write!(f, "{}", data[0])?;
            // write the tail
            <A as MvDisplayTail<M, F>>::fmt(&data[1..], f)
        }
    }
}
impl<
        U: Unsigned,
        B: Bit,
        A: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + fmt::Display,
    > MvDisplayHead<M, F> for TArr<UInt<U, B>, A>
where
    TArr<UInt<U, B>, A>: BasisSet<M>,
    A: MvDisplayHead<M, F> + MvDisplayTail<M, F>,
{
    #[inline(always)]
    fn fmt(data: &[F], f: &mut fmt::Formatter) -> fmt::Result {
        if data[0] == F::zero() {
            // keep looking for the first non-zero element
            <A as MvDisplayHead<M, F>>::fmt(&data[1..], f)
        } else {
            // write element and basis
            write!(f, "{} * {}", data[0], Basis::<UInt<U, B>, M, B0>::new())?;
            // write the tail
            <A as MvDisplayTail<M, F>>::fmt(&data[1..], f)
        }
    }
}

// Display the rest of the elements of Mvect
trait MvDisplayTail<M: Metric, F: Field> {
    fn fmt(data: &[F], f: &mut fmt::Formatter) -> fmt::Result;
}
impl<M: Metric, F: Field> MvDisplayTail<M, F> for ta![] {
    #[inline(always)]
    fn fmt(_data: &[F], _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
impl<
        L: Unsigned,
        A: BasisSet<M> + Len<Output: ArrayLength> + MvDisplayTail<M, F>,
        M: Metric,
        F: Field + fmt::Display,
    > MvDisplayTail<M, F> for ta![L | A]
where
    ta![L | A]: BasisSet<M>,
{
    #[inline(always)]
    fn fmt(data: &[F], f: &mut fmt::Formatter) -> fmt::Result {
        if data[0] > F::zero() {
            write!(f, " + {} * {}", data[0], Basis::<L, M, B0>::new())?;
        } else if data[0] < F::zero() {
            write!(f, " - {} * {}", -data[0].clone(), Basis::<L, M, B0>::new())?;
        }
        <A as MvDisplayTail<M, F>>::fmt(&data[1..], f)
    }
}
