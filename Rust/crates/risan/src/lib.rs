pub mod modular;
pub mod graph;

pub use modular::Modular;
pub use graph::{Graph, NodeId, EdgeId};

pub fn hello() {
    println!("Hello from Risan!");
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
pub fn register_module(_py: Python, m: &PyModule) -> PyResult<&PyModule> {
    let submodule = PyModule::new(_py, "risan")?;
    // Expose classes if needed.
    // For now, Graph is generic, so we might need concrete implementation/wrapper for Python.
    // Modular is also generic (const generic).
    // We can expose a few variants if needed, e.g. Modular12 (Clock).
    m.add_submodule(submodule)?;
    Ok(submodule)
}
