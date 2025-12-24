pub mod process;
pub use process::GBM;

pub fn hello() {
    println!("Hello from Sokudo!");
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
pub fn register_module(_py: Python, m: &PyModule) -> PyResult<&PyModule> {
    let submodule = PyModule::new(_py, "sokudo")?;
    m.add_class::<PyGBM>()?;
    m.add_submodule(submodule)?;
    Ok(submodule)
}

#[cfg(feature = "python")]
#[pyclass(name = "GBM")]
pub struct PyGBM {
    inner: GBM,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyGBM {
    #[new]
    fn new(mu: f64, sigma: f64, s0: f64) -> Self {
        Self { inner: GBM::new(mu, sigma, s0) }
    }
    
    fn simulate(&self, t: f64, steps: usize) -> Vec<f64> {
        self.inner.simulate(t, steps)
    }
}
