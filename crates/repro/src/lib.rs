use pyo3::prelude::*;
use repro_core::import_missing_builtin;

#[pyfunction]
pub fn repro_run(py: Python) -> PyResult<String> {
    import_missing_builtin(py)
}

#[pymodule]
pub fn _repro(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(repro_run, m)?)?;
    Ok(())
}
