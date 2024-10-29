// #![recursion_limit = "1024"]  // haven't needed it...yet
pub mod basis;
mod collector;
pub mod field;
mod marker;
pub mod metric;
pub mod mvect;
pub mod parity;
pub mod pga2d;
pub mod traits;
mod utils;
pub mod vga3d;
pub mod vga6d;

// trait GeometricAlgebra:  b mv
//     Copy              // ☑ ☑
//     + Clone           // ☑ ☑
//     + Default         // ☑ ☑
//     + Display         // ☑ ☑
//     + PartialEq       // ☑ ☑
//     + Index<Basis>    // ☒ ☑
//     + IndexMut<Basis> // ☒ ☑
//     + Add             // ☑ ☑ Addition
//     + Sub             // ☑ ☑ Subtraction
//     + BitXor          // ☑ ☑ Outer Product
//     + Mul             // ☑ ☑ Geometric Product
//     + Mul<F>          // ☑ ☑ Field Product
//     + Commutator      // ☑ ☑ Commutator
//     + ScalarProduct   // ☑ ☑ Scalar Product
//     + FatDot          // ☑ ☑ "Fat" Dot Product
//     + BitAnd          // ☑ ☑ Regressive Product
//     + BitOr           // ☑ ☑ Inner Product
//     + Shl             // ☑ ☑ Left Contraction
//     + Shr             // ☑ ☑ Right Contraction
//     + Dual + Not      // ☑ ☑ Dual
//     + Undual          // ☑ ☑ Undual
//     + Rem             // ☑ ☑ Graded
//     + Grade           // ☑ ☒ Grade
//     + Neg             // ☑ ☑ Negate
//     + Involute        // ☑ ☑ Involute
//     + Reverse         // ☑ ☑ Reverse
//     + Conjugate       // ☑ ☑ Conjugate
//     + Sandwich        // ☑ ☑ Sandwich Product
//     + Inverse         // ☐ ☐ Inverse
//     + Div             // ☐ ☐ Division
//     + Normalize       // ☑ ☐ Normalize
//     + Exponential     // ☐ ☐ Exponential
//     + Logarithm       // ☐ ☐ Logarithm

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
