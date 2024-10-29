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
//     + Commutator      // ☐ ☑ Commutator
//     + ScalarProduct   // ☐ ☑ Scalar Product
//     + FatDot          // ☐ ☑ "Fat" Dot Product
//     + BitAnd          // ☐ ☑ Regressive Product
//     + BitOr           // ☑ ☑ Inner Product
//     + Shl             // ☑ ☑ Left Contraction
//     + Shr             // ☑ ☑ Right Contraction
//     + Sandwich        // ☐ ☐ Sandwich Product
//     + Dual + Not      // ☑ ☑ Dual
//     + Undual          // ☑ ☑ Undual
//     + Rem             // ☑ ☑ Graded
//     + Grade           // ☑ ☒ Grade
//     + Neg             // ☑ ☑ Negate
//     + Involute        // ☑ ☑ Involute
//     + Reverse         // ☑ ☑ Reverse
//     + Conjugate       // ☑ ☑ Conjugate
//     + Inverse         // ☐ ☐ Inverse
//     + Div             // ☐ ☐ Division
//     + Normalize       // ☑ ☐ Normalize
//     + Exponential     // ☐ ☐ Exponential
//     + Logarithm       // ☐ ☐ Logarithm
// {}
