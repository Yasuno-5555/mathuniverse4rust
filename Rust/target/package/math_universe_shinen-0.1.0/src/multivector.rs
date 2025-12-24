use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// MultiVector in Cl(3,0) Geometric Algebra.
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MultiVector {
    pub coeffs: [f64; 8],
}

impl MultiVector {
    pub const ZERO: Self = Self { coeffs: [0.0; 8] };
    pub const ONE: Self = Self { coeffs: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };

    pub fn new(coeffs: [f64; 8]) -> Self {
        Self { coeffs }
    }

    /// Reversion (~): Reverses the order of vectors in the product.
    /// scalar, vector -> same
    /// bivector -> negative
    /// trivector -> negative
    pub fn reverse(self) -> Self {
        let mut c = self.coeffs;
        // 0 (scalar): +
        // 1,2,3 (vector): +
        // 4,5,6 (bivector): -
        c[4] = -c[4];
        c[5] = -c[5];
        c[6] = -c[6];
        // 7 (trivector): - (e3e2e1 = -e1e2e3)
        c[7] = -c[7];
        Self { coeffs: c }
    }

    /// Rotate this multivector (v) by rotor (R): R * v * R~
    pub fn rotate(self, rotor: Self) -> Self {
        rotor * self * rotor.reverse()
    }
}

// Ops impl
impl Add for MultiVector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut res = Self::ZERO;
        for i in 0..8 {
            res.coeffs[i] = self.coeffs[i] + rhs.coeffs[i];
        }
        res
    }
}

impl Sub for MultiVector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut res = Self::ZERO;
        for i in 0..8 {
            res.coeffs[i] = self.coeffs[i] - rhs.coeffs[i];
        }
        res
    }
}

impl Neg for MultiVector {
    type Output = Self;
    fn neg(self) -> Self {
        let mut res = Self::ZERO;
        for i in 0..8 {
            res.coeffs[i] = -self.coeffs[i];
        }
        res
    }
}

impl Mul for MultiVector {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let a = self.coeffs;
        let b = rhs.coeffs;
        let mut c = [0.0; 8];
        c[0] = a[0]*b[0] + a[1]*b[1] + a[2]*b[2] + a[3]*b[3] - a[4]*b[4] - a[5]*b[5] - a[6]*b[6] - a[7]*b[7];
        c[1] = a[0]*b[1] + a[1]*b[0] - a[2]*b[4] + a[3]*b[6] + a[4]*b[2] - a[6]*b[3] - a[7]*b[5] - a[5]*b[7];
        c[2] = a[0]*b[2] + a[1]*b[4] + a[2]*b[0] - a[3]*b[5] - a[4]*b[1] + a[5]*b[3] - a[7]*b[6] - a[6]*b[7];
        c[3] = a[0]*b[3] - a[1]*b[6] + a[2]*b[5] + a[3]*b[0] + a[6]*b[1] - a[5]*b[2] - a[7]*b[4] - a[4]*b[7];
        c[4] = a[0]*b[4] + a[1]*b[2] - a[2]*b[1] + a[4]*b[0] - a[6]*b[5] + a[5]*b[6] + a[3]*b[7] + a[7]*b[3]; 
        c[5] = a[0]*b[5] + a[2]*b[3] - a[3]*b[2] + a[5]*b[0] - a[4]*b[6] + a[6]*b[4] + a[1]*b[7] + a[7]*b[1];
        c[6] = a[0]*b[6] + a[3]*b[1] - a[1]*b[3] + a[6]*b[0] - a[5]*b[4] + a[4]*b[5] + a[2]*b[7] + a[7]*b[2];
        c[7] = a[0]*b[7] + a[7]*b[0] + a[1]*b[5] + a[2]*b[6] + a[3]*b[4] + a[5]*b[1] + a[6]*b[2] + a[4]*b[3];
        Self { coeffs: c }
    }
}

// Display
impl fmt::Display for MultiVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.coeffs)
    }
}
