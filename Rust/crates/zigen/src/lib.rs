pub mod scalar;
pub mod dual;

pub use scalar::Scalar;
pub use dual::Dual;

pub fn hello() {
    println!("Hello from Zigen!");
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
pub fn register_module(_py: Python, m: &PyModule) -> PyResult<&PyModule> {
    let submodule = PyModule::new(_py, "zigen")?;
    // We cannot expose generic Dual<T> directly.
    // We should expose a concrete F64 version.
    m.add_class::<DualF64>()?;
    m.add_submodule(submodule)?;
    Ok(submodule)
}

#[cfg(feature = "python")]
#[pyclass(name = "Dual")]
#[derive(Clone, Copy)]
pub struct DualF64 {
    pub inner: Dual<f64>,
}

#[cfg(feature = "python")]
#[pymethods]
impl DualF64 {
    #[new]
    fn new(real: f64, dual: f64) -> Self {
        DualF64 {
            inner: Dual::new(real, dual),
        }
    }

    #[getter]
    fn real(&self) -> f64 { self.inner.real }

    #[getter]
    fn dual(&self) -> f64 { self.inner.dual }
    
    fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }
    
    fn __add__(&self, rhs: &DualF64) -> DualF64 {
        DualF64 { inner: self.inner + rhs.inner }
    }
    
    fn __sub__(&self, rhs: &DualF64) -> DualF64 {
        DualF64 { inner: self.inner - rhs.inner }
    }
    
    fn __mul__(&self, rhs: &DualF64) -> DualF64 {
        DualF64 { inner: self.inner * rhs.inner }
    }
    
    fn __truediv__(&self, rhs: &DualF64) -> DualF64 {
        DualF64 { inner: self.inner / rhs.inner }
    }
    
    fn sin(&self) -> DualF64 { DualF64 { inner: self.inner.sin() } }
    fn cos(&self) -> DualF64 { DualF64 { inner: self.inner.cos() } }
    fn exp(&self) -> DualF64 { DualF64 { inner: self.inner.exp() } }
}
