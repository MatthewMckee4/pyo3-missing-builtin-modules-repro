use pyo3::prelude::*;
use repro_core::import_missing_builtin;

fn main() {
    Python::with_gil(|py| {
        import_missing_builtin(py).unwrap();
    });
}
