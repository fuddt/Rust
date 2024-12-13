use pyo3::prelude::*;


// #[pyfunction]: Rustの関数をPythonから利用可能にするマクロ。
// #[pymodule]: PythonのモジュールをRustで定義するためのマクロ。

/// This module provides basic functions for demonstration.
/// Add the `#[pyfunction]` attribute to make a Rust function accessible from Python.
#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Python module definition
#[pymodule]
fn pylib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}