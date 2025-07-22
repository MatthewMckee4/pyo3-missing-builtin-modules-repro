use pyo3::{ffi::c_str, PyResult, Python};

pub fn import_missing_builtin(py: Python) -> PyResult<String> {
    py.run(c_str!("import pytest; print(pytest)"), None, None)
        .unwrap();
    Ok("ok".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_import_missing_builtin() {
        Python::with_gil(|py| {
            import_missing_builtin(py).unwrap();
        });
    }
}
