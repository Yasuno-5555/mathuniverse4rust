use crate::scalar::Scalar;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt;

/// Dual number implementation for Forward Mode Automatic Differentiation.
/// 
/// A dual number is defined as `a + bε` where `a` is the real part, `b` is the dual (infinitesimal) part,
/// and `ε^2 = 0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dual<T: Scalar> {
    pub real: T,
    pub dual: T,
}

impl<T: Scalar> Dual<T> {
    /// Create a new dual number.
    pub fn new(real: T, dual: T) -> Self {
        Self { real, dual }
    }

    /// Create a constant dual number (dual part is zero).
    pub fn constant(val: T) -> Self {
        Self { real: val, dual: T::zero() }
    }

    /// Create a variable for differentiation (real part `val`, dual part `1`).
    pub fn variable(val: T) -> Self {
        Self { real: val, dual: T::one() }
    }
}

// Display
impl<T: Scalar> fmt::Display for Dual<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}ε", self.real, self.dual)
    }
}

// Operators
impl<T: Scalar> Add for Dual<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            dual: self.dual + rhs.dual,
        }
    }
}

impl<T: Scalar> Sub for Dual<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            real: self.real - rhs.real,
            dual: self.dual - rhs.dual,
        }
    }
}

impl<T: Scalar> Mul for Dual<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            real: self.real * rhs.real,
            dual: self.real * rhs.dual + self.dual * rhs.real,
        }
    }
}

impl<T: Scalar> Div for Dual<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        // (a + bε) / (c + dε) = (a/c) + ((bc - ad)/c^2)ε
        let inv_c = T::one() / rhs.real;
        Self {
            real: self.real * inv_c,
            dual: (self.dual * rhs.real - self.real * rhs.dual) * inv_c * inv_c,
        }
    }
}

impl<T: Scalar> Neg for Dual<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            real: -self.real,
            dual: -self.dual,
        }
    }
}

// Mathematical Functions (Sin, Cos, etc.) using chain rule
impl<T: Scalar> Dual<T> {
    pub fn sin(self) -> Self {
        Self {
            real: self.real.sin(),
            dual: self.dual * self.real.cos(),
        }
    }

    pub fn cos(self) -> Self {
        Self {
            real: self.real.cos(),
            dual: -self.dual * self.real.sin(),
        }
    }

    pub fn exp(self) -> Self {
        let exp_real = self.real.exp();
        Self {
            real: exp_real,
            dual: self.dual * exp_real,
        }
    }
    
    pub fn powf(self, n: T) -> Self {
        // x^n -> (x.real^n, n * x.real^(n-1) * x.dual)
        let pow_real = self.real.powf(n);
        // If real is 0, derivative issues might occur, assuming safe domain for now
        // d/dx (x^n) = n * x^(n-1)
        // new dual = dual * n * real^(n-1)
        // simpler: n * real^(n-1) = n * pow_real / real
        let deriv = n * self.real.powf(n - T::one());
        Self {
            real: pow_real,
            dual: self.dual * deriv,
        }
    }
}

// Python interop (impl via separate pyclass struct in wrapper usually, 
// but if we want Dual to be pyclass, we need PyO3 dep in Zigen.
// The plan said "Python feature flags" in main crate?
// If Dual is defined here, and we want to expose it to Python, we have 2 options:
// 1. Add optional pyo3 dependency to zigen and use #[pyclass] here.
// 2. Wrap it in `math_universe` crate.
// The user request said "Decorate structs and functions with #[pyclass]".
// It is better to have it in the crate itself behind feature flag.
// So I should update Zigen Cargo.toml to add optional pyo3.

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymethods]
impl<T: Scalar + Into<f64> + From<f64>> Dual<T> {
    // PyO3 limitation: generics on pyclass are hard.
    // Usually we expose `DualF64` to Python.
}
