pub mod multivector;
pub use multivector::MultiVector;

pub fn hello() {
    println!("Hello from Shinen!");
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
pub fn register_module(_py: Python, m: &PyModule) -> PyResult<&PyModule> {
    let submodule = PyModule::new(_py, "shinen")?;
    m.add_class::<MultiVector>()?;
    m.add_submodule(submodule)?;
    Ok(submodule)
}

#[cfg(feature = "python")]
#[pymethods]
impl MultiVector {
    #[new]
    fn py_new(coeffs: Vec<f64>) -> Self {
        let mut c = [0.0; 8];
        for (i, v) in coeffs.iter().enumerate().take(8) {
            c[i] = *v;
        }
        Self { coeffs: c }
    }
    
    fn __repr__(&self) -> String {
        format!("{}", self)
    }

    fn __add__(&self, rhs: MultiVector) -> MultiVector { *self + rhs }
    fn __sub__(&self, rhs: MultiVector) -> MultiVector { *self - rhs }
    fn __mul__(&self, rhs: MultiVector) -> MultiVector { *self * rhs }
}

// We need to add #[pyclass] to MultiVector in multivector.rs, OR wrap it here.
// Since MultiVector is not generic, we can put #[pyclass] directly on it in multivector.rs if we add pyo3 dependency to shinen.
// I will wrap it here if I didn't add pyo3 to shinen.
// But I should add pyo3 to shinen as well.
