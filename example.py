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
    def _my_str_len(ctx, x: pa.lib.StringArray):
        return udf.get_str_len(x)
        # return pa.array((len(str(s)) for s in x), type=pa.uint32())

    pc.register_vector_function(
        _my_str_len,
        "my_str_len",  # name
        {
            "summary": "gets string length",
            "description": "Given a string 'x' returns the length of x",
        },
        {
            "x": pa.string(),
        },
        pa.uint32(),
    )

    # res = pc.call_function("my_str_len", [pa.array(["foo", "bar", "quz"])])
    # print(res)

    # in memory database
    with duckdb.connect(":memory:") as conn:
        conn.create_function(
            "my_str_len", my_str_len_udf, [t.VARCHAR], t.UINTEGER, type="arrow"
        )
        conn.sql("create table test(s varchar)")
        conn.sql("insert into test values ('foo'),('bar'), (NULL), ('barx')")
        res = conn.sql("select s, my_str_len(s) as l  from test")
        print(res)
        # str_arr = pa.array(str_chunks)
        # lengths_arr = udf.get_str_len(str_arr)
        # for (s, l) in zip(str_arr, lengths_arr):
        #     print(f"{s}->{l}")


if __name__ == "__main__":
    main()
