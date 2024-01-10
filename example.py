import udf
import time

import pyarrow as pa
import duckdb


def main():
    # in memory database
    with duckdb.connect(":memory:") as conn:
        conn.sql("create table test(s varchar)")
        conn.sql("insert into test values ('foo'),('bar'), (NULL), ('barx')")
        str_chunks = conn.sql("select * from test").arrow()["s"]
        str_arr = pa.array(str_chunks)
        lengths_arr = udf.get_str_len(str_arr)
        for (s, l) in zip(str_arr, lengths_arr):
            print(f"{s}->{l}")


if __name__ == "__main__":
    main()
