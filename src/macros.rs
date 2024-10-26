/// just typenum::tarr but with elixir style [ a, b, c | rest ] syntax
#[macro_export]
macro_rules! ta {
    () => ( typenum::ATerm );
    ($n:ty | $tail:ty ) => ( typenum::TArr<$n, $tail>);
    ($n:ty, $($tail:ty),+ | $rest:ty) => ( typenum::TArr<$n, ta![$($tail),+ | $rest]>);
    ($n:ty) => ( typenum::TArr<$n, typenum::ATerm> );
    ($n:ty,) => ( typenum::TArr<$n, typenum::ATerm> );
    ($n:ty, $($tail:ty),+) => ( typenum::TArr<$n, ta![$($tail),+]> );
    ($n:ty, $($tail:ty),+,) => ( typenum::TArr<$n, ta![$($tail),+]> );
}
