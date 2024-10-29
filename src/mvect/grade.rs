use crate::{
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
    ta,
    utils::{count::CountOf, Branch, If},
};
use generic_array::ArrayLength;
use typenum::{Bit, Eq, IsEqual, Len, TypeArray, Unsigned, B1};

impl<
        G: Unsigned,
        A: BasisSet<M>
            + Len<Output: ArrayLength>
            + GradedBs<G, F, Output: BasisSet<M> + Len<Output: ArrayLength>>,
        M: Metric,
        F: Field,
    > core::ops::Rem<G> for Mvect<A, M, F>
{
    type Output = Mvect<<A as GradedBs<G, F>>::Output, M, F>;
    #[inline(always)]
    fn rem(self, _: G) -> Self::Output {
        let mut out = Self::Output::default();
        <A as GradedBs<G, F>>::grader(&mut out.0, &self.0);
        out
    }
}

pub trait GradedBs<G: Unsigned, F: Field> {
    type Output: TypeArray;
    fn grader(out: &mut [F], data: &[F]);
}

impl<G: Unsigned, F: Field> GradedBs<G, F> for ta![] {
    type Output = ta![];
    #[inline(always)]
    fn grader(_: &mut [F], _: &[F]) {}
}

impl<
        G: Unsigned
            + IsEqual<
                U::Count,
                Output: Branch<
                    ta![U | <A as GradedBs<G, F>>::Output],
                    <A as GradedBs<G, F>>::Output,
                    Output: TypeArray,
                >,
            >,
        U: Unsigned + CountOf<B1>,
        A: TypeArray + GradedBs<G, F>,
        F: Field,
    > GradedBs<G, F> for ta![U | A]
{
    type Output =
        If<Eq<G, U::Count>, ta![U | <A as GradedBs<G, F>>::Output], <A as GradedBs<G, F>>::Output>;
    #[inline(always)]
    fn grader(out: &mut [F], data: &[F]) {
        if Eq::<G, U::Count>::BOOL {
            out[0] = data[0].clone();
            <A as GradedBs<G, F>>::grader(&mut out[1..], &data[1..]);
        } else {
            <A as GradedBs<G, F>>::grader(out, &data[1..]);
        }
    }
}
