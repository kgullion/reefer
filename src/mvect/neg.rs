use crate::{
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
    parity::{
        ConjugatePar, ConjugateParity, InvolutePar, InvoluteParity, ReversePar, ReverseParity,
    },
    ta,
    traits::{Conjugate, Involute, Reverse},
};
use core::ops::Neg;
use generic_array::ArrayLength;
use typenum::{Bit, Len, TypeArray, Unsigned};

// -------------------------------------------------------------------------------------
// Negation
impl<A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field + Neg> Neg for Mvect<A, M, F> {
    type Output = Mvect<A, M, F>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        let mut data = self.0.clone();
        for elem in data.iter_mut() {
            *elem = -elem.clone();
        }
        Self::Output::new(data)
    }
}

// -------------------------------------------------------------------------------------
// Involution
impl<A: BasisSet<M> + Len<Output: ArrayLength> + InvoluteBs<F>, M: Metric, F: Field> Involute
    for Mvect<A, M, F>
{
    type Output = Mvect<A, M, F>;
    #[inline(always)]
    fn involute(self) -> Self::Output {
        let mut data = self.0.clone();
        <A as InvoluteBs<F>>::involute(&mut data);
        Self::Output::new(data)
    }
}

trait InvoluteBs<F: Field>: TypeArray {
    fn involute(data: &mut [F]);
}
impl<F: Field> InvoluteBs<F> for ta![] {
    #[inline(always)]
    fn involute(_: &mut [F]) {}
}
impl<U: Unsigned + InvolutePar, A: InvoluteBs<F>, F: Field> InvoluteBs<F> for ta![U | A] {
    #[inline(always)]
    fn involute(data: &mut [F]) {
        if InvoluteParity::<U>::BOOL {
            data[0] = -data[0].clone();
        }
        <A as InvoluteBs<F>>::involute(&mut data[1..]);
    }
}

// -------------------------------------------------------------------------------------
// Reverse
impl<A: BasisSet<M> + Len<Output: ArrayLength> + ReverseBs<F>, M: Metric, F: Field> Reverse
    for Mvect<A, M, F>
{
    type Output = Mvect<A, M, F>;
    #[inline(always)]
    fn reverse(self) -> Self::Output {
        let mut data = self.0.clone();
        <A as ReverseBs<F>>::reverse(&mut data);
        Self::Output::new(data)
    }
}

trait ReverseBs<F: Field>: TypeArray {
    fn reverse(data: &mut [F]);
}
impl<F: Field> ReverseBs<F> for ta![] {
    #[inline(always)]
    fn reverse(_: &mut [F]) {}
}
impl<U: Unsigned + ReversePar, A: ReverseBs<F>, F: Field> ReverseBs<F> for ta![U | A] {
    #[inline(always)]
    fn reverse(data: &mut [F]) {
        if ReverseParity::<U>::BOOL {
            data[0] = -data[0].clone();
        }
        <A as ReverseBs<F>>::reverse(&mut data[1..]);
    }
}

// -------------------------------------------------------------------------------------
// Conjugation
impl<A: BasisSet<M> + Len<Output: ArrayLength> + ConjugateBs<F>, M: Metric, F: Field> Conjugate
    for Mvect<A, M, F>
{
    type Output = Mvect<A, M, F>;
    #[inline(always)]
    fn conjugate(self) -> Self::Output {
        let mut data = self.0.clone();
        <A as ConjugateBs<F>>::conjugate(&mut data);
        Self::Output::new(data)
    }
}

trait ConjugateBs<F: Field>: TypeArray {
    fn conjugate(data: &mut [F]);
}
impl<F: Field> ConjugateBs<F> for ta![] {
    #[inline(always)]
    fn conjugate(_: &mut [F]) {}
}
impl<U: Unsigned + ConjugatePar, A: ConjugateBs<F>, F: Field> ConjugateBs<F> for ta![U | A] {
    #[inline(always)]
    fn conjugate(data: &mut [F]) {
        if ConjugateParity::<U>::BOOL {
            data[0] = -data[0].clone();
        }
        <A as ConjugateBs<F>>::conjugate(&mut data[1..]);
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use crate::{
        basis::Basis,
        field::Field,
        metric::Metric,
        mvect::{basis_set::BasisSet, Mvect},
        pga2d::{e0, e01, e012, e02, e1, e12, e2, scalar as e},
        ta,
    };
    use typenum::{tarr, B0, P1, U0, U1, U2, U3, U4, U5, U6, U7, Z0};

    #[test]
    fn test_neg() {
        let a = 1.0 * e
            + 2.0 * e0
            + 3.0 * e1
            + 5.0 * e2
            + 7.0 * e01
            + 11.0 * e02
            + 13.0 * e12
            + 17.0 * e012;
        let b = -1.0 * e
            - 2.0 * e0
            - 3.0 * e1
            - 5.0 * e2
            - 7.0 * e01
            - 11.0 * e02
            - 13.0 * e12
            - 17.0 * e012;
        assert!(-a == b);
        assert!(a == -b);
    }
}
