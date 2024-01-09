use std::{ptr::NonNull, sync::Arc};

use arrow::pyarrow::{FromPyArrow, ToPyArrow};
use eyre::{Context, ContextCompat, Result};
use pyo3::prelude::*;

#[pyfunction]
fn echo_array<'a>(py: Python, a: &PyAny) -> Result<Py<PyAny>> {
    let arraydata =
        arrow::array::ArrayData::from_pyarrow(a).context("Could not convert arrow data")?;
    let buffer = arraydata.buffers()[0].as_slice();
    let len = buffer.len();

    let arc_s = Arc::new(buffer.to_vec());
    let ptr = NonNull::new(arc_s.as_ptr() as *mut _).context("Could not create pointer")?;
    let raw_buffer = unsafe { arrow::buffer::Buffer::from_custom_allocation(ptr, len, arc_s) };
    let output = arrow::array::ArrayData::try_new(
        arrow::datatypes::DataType::UInt8,
        len,
        None,
        0,
        vec![raw_buffer],
        vec![],
    )
    .context("could not create arrow arraydata")?;
    output
        .to_pyarrow(py)
        .context("Could not convert to pyarrow")
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn udf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(echo_array, m)?)?;

    Ok(())
}
