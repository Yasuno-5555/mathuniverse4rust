pub use math_universe_zigen as zigen;
pub use math_universe_shinen as shinen;
pub use math_universe_risan as risan;
pub use math_universe_sokudo as sokudo;
pub use math_universe_keirin as keirin;
pub use math_universe_ryoshi as ryoshi;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn mathuniverse_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_submodule(zigen::register_module(_py)?)?;
    m.add_submodule(shinen::register_module(_py)?)?;
    m.add_submodule(risan::register_module(_py)?)?;
    m.add_submodule(ryoshi::register_module(_py)?)?;
    m.add_submodule(sokudo::register_module(_py)?)?;
    m.add_submodule(keirin::register_module(_py)?)?;
    Ok(())
}
