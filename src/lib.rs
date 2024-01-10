#![allow(dead_code)]
#![allow(unused_imports)]
use std::{ptr::NonNull, sync::Arc};

use arrow::array::Array;
use arrow::array::{self as arr, StringArray};
use arrow::pyarrow::{FromPyArrow, ToPyArrow};
use eyre::{Context, ContextCompat, Result};
use pyo3::prelude::*;

#[pyfunction]
fn get_str_len<'a>(py: Python, a: &PyAny) -> Result<Py<PyAny>> {
    let arraydata =
        arrow::array::ArrayData::from_pyarrow(a).context("Could not convert arrow data")?;

    // get string lengths
    let strs = StringArray::from(arraydata);
    let lengths_arr = {
        let mut arr_builder = arr::UInt32Builder::with_capacity(strs.len());
        strs.iter().for_each(|v| {
            if let Some(s) = v {
                arr_builder.append_value(s.len() as u32);
            } else {
                arr_builder.append_null();
            }
        });
        arr_builder.finish()
    };
    let output = lengths_arr.to_data();

    output
        .to_pyarrow(py)
        .context("Could not convert to pyarrow")
}

#[pyfunction]
fn read_pyarrow_table<'a>(_py: Python, a: &PyAny) {
    // a: pyarrow.lib.RecordBatchReader
    // let arraydata = arrow::array::ArrayData::from_pyarrow(a).unwrap();
    let reader = arrow::ffi_stream::ArrowArrayStreamReader::from_pyarrow(a).unwrap();
    for chunk in reader {
        let batch = chunk.expect("Should be valid RecordBatch");
        println!("row: {:?}", batch.num_rows());
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn udf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_str_len, m)?)?;
    m.add_function(wrap_pyfunction!(read_pyarrow_table, m)?)?;

    Ok(())
}
