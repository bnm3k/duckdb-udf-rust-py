use pyo3::prelude::*;

#[pyfunction]
fn do_sth() {
    println!("doing sthing udf function");
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn udf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(do_sth, m)?)?;
    Ok(())
}
