# Rust-based vectorized UDFs for DuckDB

Starter code for implementing vectorized UDFs in Rust that can be used in Python
and DuckDB.

## Installation

Setup a virtual environment:

```
python3 -m venv .venv
source .venv/bin/activate
```

Install maturin:

```
pup install maturin
```

Install python dependencies:

```
pip install -r requirements.txt
```

## Usage

Implement the vectorized UDF in Rust and use PyO3 to generate the Python
bindings. You can use the following
[Rust-Python FFI blog post](https://dora.carsmos.ai/blog/rust-python/) as a
starting point.

Suppose the UDF is `get_str_len` that takes in a vector of strings and returns a
vector of string lengths. From there, register it as `pyarrow.compute` function:

```python
import pyarrow as pa
import pyarrow.compute as pc

pc.register_vector_function(
    lambda ctx, x: udf.get_str_len(x),  # rust-based function
    "my_str_len",  # name
    {  # doc
        "summary": "gets string length",
        "description": "Given a string 'x' returns the length of x",
    },
    {
        "x": pa.string(),  # input
    },
    pa.uint32(),
)
```

After that, register the function within the DuckDB Conn:

```python
import duckdb
import duckdb.typing as t

def my_str_len_udf(x: pa.lib.ChunkedArray):
    return pc.call_function("my_str_len", [x])

conn = duckdb(":memory:")
conn.create_function(
    "my_str_len", my_str_len_udf, [t.VARCHAR], t.UINTEGER, type="arrow"
)
```

From there, you can use the UDF in SQL:

```python
conn.sql("create table test(s varchar)")
conn.sql("insert into test select range::varchar from range(0,2000000)")
res = conn.sql("select s, my_str_len(s) as l  from test")
```
