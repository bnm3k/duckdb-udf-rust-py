import udf
import time

import pyarrow as pa
import pyarrow.compute as pc
import duckdb
import duckdb.typing as t


def my_str_len_udf(x: pa.lib.ChunkedArray):
    return pc.call_function("my_str_len", [x])


def main():
    # register UDF
    # def _my_str_len(ctx, x: pa.lib.StringArray):
    #     return udf.get_str_len(x)

    pc.register_vector_function(
        lambda ctx, x: udf.get_str_len(x),  # function
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

    # in memory database
    with duckdb.connect(":memory:") as conn:
        conn.create_function(
            "my_str_len", my_str_len_udf, [t.VARCHAR], t.UINTEGER, type="arrow"
        )
        conn.sql("create table test(s varchar)")
        conn.sql("insert into test select range::varchar from range(0,2000000)")
        res = conn.sql("select s, my_str_len(s) as l  from test")
        print(res)

        # tbl = conn.sql("select s from test").arrow()
        # reader: pa.lib.RecordBatchReader = tbl.to_reader()
        # udf.read_pyarrow_table(reader)


if __name__ == "__main__":
    main()
