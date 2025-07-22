use pyo3::{ffi::c_str, types::PyAnyMethods, PyResult, Python};

pub fn current_python_version() -> (u8, u8) {
    Python::with_gil(|py| {
        let inferred_python_version = py.version_info();
        (inferred_python_version.major, inferred_python_version.minor)
    })
}

pub fn get_executable_path() -> PyResult<String> {
    Python::with_gil(|py| {
        let sys_path = py.import("sys")?;
        let executable_path = sys_path.getattr("executable")?;
        executable_path.extract::<String>()
    })
}

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

    #[test]
    fn test_current_python_version() {
        assert_eq!(current_python_version(), (3, 12));
    }

    #[test]
    fn test_get_executable_path() {
        assert_eq!(get_executable_path().unwrap(), "/usr/bin/python3");
    }
}
