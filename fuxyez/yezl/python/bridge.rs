//! Python FFI Bridge for Fuxyez
//! Enables .fuxpy file execution

use pyo3::prelude::*;
use pyo3::types::PyDict;

pub struct PythonBridge {
    interpreter: Py<PyAny>,
}

impl PythonBridge {
    pub fn new() -> PyResult<Self> {
        Python::with_gil(|py| {
            let interpreter = py.eval("__import__('sys')", None, None)?;
            Ok(Self {
                interpreter: interpreter.into(),
            })
        })
    }

    pub fn execute_fuxpy(&self, code: &str) -> PyResult<String> {
        Python::with_gil(|py| {
            let locals = PyDict::new(py);
            
            // Execute Python code
            py.run(code, None, Some(locals))?;
            
            // Return result
            Ok("Success".to_string())
        })
    }

    pub fn import_module(&self, module: &str) -> PyResult<Py<PyAny>> {
        Python::with_gil(|py| {
            let module = py.import(module)?;
            Ok(module.into())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_bridge() {
        let bridge = PythonBridge::new().unwrap();
        let result = bridge.execute_fuxpy("print('Hello from Python')");
        assert!(result.is_ok());
    }
}
