use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn sum(a: usize, b: usize) -> PyResult<usize> {
    Ok(a + b)
}

#[pyfunction]
fn adjust_r2(r2: f64, n: f64, p: f64) -> PyResult<f64> {
    Ok(1.0 - (1.0 - r2) * ((n - 1.0) / (n - p - 1.0)))
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn funkybox(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(sum))?;
    m.add_wrapped(wrap_pyfunction!(adjust_r2))?;

    Ok(())
}
