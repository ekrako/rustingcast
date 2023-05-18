use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn fibonacci(n: u64) -> u64 {
    match n {
        1 | 0 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[pyfunction]
fn helloworld(name:String)-> String {
    return format!("Hello, {}! Nice to meet you.", name);
}

/// A Python module implemented in Rust.
#[pymodule]
fn fibrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(helloworld, m)?)?;
    Ok(())
}

