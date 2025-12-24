use num_traits::{Float, FromPrimitive, Num, NumAssign, NumOps, One, Zero};
use std::fmt::{Debug, Display};
use std::iter::{Product, Sum};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};

/// A trait representing a scalar value that can be used in Zigen for dual numbers.
///
/// This trait aggregates common numerical traits required for mathematical operations.
/// It is designed to allow `Dual<T>` to work with standard floating point types (f32, f64)
/// as well as potentially other compatible types in the future.
pub trait Scalar:
    Copy
    + Clone
    + Debug
    + Display
    + PartialEq
    + PartialOrd
    + Zero
    + One
    + Neg<Output = Self>
    + Num
    + NumOps
    + NumAssign
    + Sum
    + Product
    + FromPrimitive
    + 'static
{
    // You can add custom methods here if needed, e.g. mathematical functions
    // if not already covered by Num/Float traits.
    
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn powf(self, n: Self) -> Self;
}

// Implement Scalar for f32 and f64
impl Scalar for f32 {
    fn abs(self) -> Self { Float::abs(self) }
    fn sqrt(self) -> Self { Float::sqrt(self) }
    fn sin(self) -> Self { Float::sin(self) }
    fn cos(self) -> Self { Float::cos(self) }
    fn exp(self) -> Self { Float::exp(self) }
    fn ln(self) -> Self { Float::ln(self) }
    fn powf(self, n: Self) -> Self { Float::powf(self, n) }
}

impl Scalar for f64 {
    fn abs(self) -> Self { Float::abs(self) }
    fn sqrt(self) -> Self { Float::sqrt(self) }
    fn sin(self) -> Self { Float::sin(self) }
    fn cos(self) -> Self { Float::cos(self) }
    fn exp(self) -> Self { Float::exp(self) }
    fn ln(self) -> Self { Float::ln(self) }
    fn powf(self, n: Self) -> Self { Float::powf(self, n) }
}
