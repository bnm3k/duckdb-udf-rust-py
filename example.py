import udf
import time

import pyarrow as pa


def main():
    strs = ["foo", "bar", None, "barx"]
    str_arr = pa.array(strs, type=pa.string())
    lengths_arr = udf.get_str_len(str_arr)
    for (s, l) in zip(str_arr, lengths_arr):
        print(f"{s}->{l}")


if __name__ == "__main__":
    main()
