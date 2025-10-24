use pyo3::Python;
use repro_core::import_missing_builtin;

fn main() {
    Python::with_gil(|py| match import_missing_builtin(py) {
        Ok(_) => println!("Import successful"),
        Err(e) => println!("Import failed: {}", e),
    });
}
