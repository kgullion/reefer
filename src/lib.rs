// #![recursion_limit = "1024"]  // haven't needed it...yet
pub mod basis;
mod collector;
pub mod field;
mod macros;
pub mod metric;
pub mod mvect;
pub mod pga2d;
pub mod traits;
mod utils;

use basis::{Basis, ZeroVect};
use field::Field;
use generic_array::ArrayLength;
use metric::Metric;
use mvect::{basis_set::BasisSet, Mvect};
use typenum::{Bit, Len, Unsigned};

trait GeometricObject: Default {}
impl GeometricObject for ZeroVect {}
impl<U: Unsigned, M: Metric, S: Bit> GeometricObject for Basis<U, M, S> {}
impl<A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field> GeometricObject
    for Mvect<A, M, F>
{
}

// trait GeometricAlgebra:  b mv
//     Copy              // ☑ ☑
//     + Clone           // ☑ ☑
//     + Default         // ☑ ☑
//     + Display         // ☑ ☑
//     + PartialEq       // ☑ ☑
//     + Index<Basis>    // ☐ ☐
//     + IndexMut<Basis> // ☒ ☐
//     + Add             // ☐ ☑ Addition
//     + Sub             // ☐ ☑ Subtraction
//     + BitXor          // ☑ ☑ Outer Product
//     + Mul             // ☑ ☑ Geometric Product
//     + Mul<F>          // ☑ ☑ Field Product
//     + Commutator      // ☐ ☑ Commutator
//     + ScalarProduct   // ☐ ☑ Scalar Product
//     + FatDot          // ☐ ☑ "Fat" Dot Product
//     + BitAnd          // ☐ ☐ Regressive Product
//     + BitOr           // ☑ ☑ Inner Product
//     + Shl             // ☑ ☑ Left Contraction
//     + Shr             // ☑ ☑ Right Contraction
//     + Rem             // ☐ ☐
//     + Fn(Rhs) -> Out  // ☐ ☐ Sandwich Product
//     + Dual + Not      // ☑ ☐ Dual
//     + Undual          // ☑ ☐ Undual
//     + Grade           // ☑ ☐ Grade
//     + Neg             // ☑ ☐ Negate
//     + Involute        // ☑ ☐ Involute
//     + Reverse         // ☑ ☐ Reverse
//     + Conjugate       // ☑ ☐ Conjugate
//     + Inverse         // ☑ ☐ Inverse
//     + Div             // ☐ ☐ Division
//     + Normalize       // ☑ ☐ Normalize
//     + Exponential     // ☐ ☐ Exponential
//     + Logarithm       // ☐ ☐ Logarithm
// {}
