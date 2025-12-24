pub mod tda;
pub use tda::PointCloud;

pub fn hello() {
    println!("Hello from Keirin!");
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
pub fn register_module(_py: Python, m: &PyModule) -> PyResult<&PyModule> {
    let submodule = PyModule::new(_py, "keirin")?;
    m.add_class::<PyPointCloud>()?;
    m.add_submodule(submodule)?;
    Ok(submodule)
}

#[cfg(feature = "python")]
#[pyclass(name = "PointCloud")]
pub struct PyPointCloud {
    inner: PointCloud,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyPointCloud {
    #[new]
    fn new(points: Vec<Vec<f64>>) -> Self {
        Self { inner: PointCloud::new(points) }
    }
}
