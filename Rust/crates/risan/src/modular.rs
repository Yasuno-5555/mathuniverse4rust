use std::ops::{Add, Mul, Sub};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modular<const M: i64>(pub i64);

impl<const M: i64> Modular<M> {
    pub fn new(val: i64) -> Self {
        let rem = val % M;
        Self(if rem < 0 { rem + M } else { rem })
    }
    
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl<const M: i64> Add for Modular<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}

impl<const M: i64> Sub for Modular<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0)
    }
}

impl<const M: i64> Mul for Modular<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.0)
    }
}

impl<const M: i64> fmt::Display for Modular<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
