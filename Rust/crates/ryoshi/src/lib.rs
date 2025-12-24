pub mod quantum_state;
pub use quantum_state::QuantumState;

pub fn hello() {
    println!("Hello from Ryoshi!");
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
pub fn register_module(_py: Python, m: &PyModule) -> PyResult<&PyModule> {
    let submodule = PyModule::new(_py, "ryoshi")?;
    m.add_class::<PyQuantumState>()?;
    m.add_submodule(submodule)?;
    Ok(submodule)
}

#[cfg(feature = "python")]
#[pyclass(name = "QuantumState")]
pub struct PyQuantumState {
    inner: QuantumState,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyQuantumState {
    #[new]
    fn new(num_qubits: usize) -> Self {
        Self { inner: QuantumState::new(num_qubits) }
    }
    
    fn apply_x(&mut self, target: usize) {
        self.inner.apply_x(target);
    }
    
    fn apply_h(&mut self, target: usize) {
        self.inner.apply_h(target);
    }
    
    fn apply_cnot(&mut self, control: usize, target: usize) {
        self.inner.apply_cnot(control, target);
    }
    
    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}
