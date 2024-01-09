import udf
import time

import pyarrow as pa


def main():
    some_list = pa.array((i for i in range(3)), type=pa.uint8())
    got = udf.echo_array(some_list)
    print(got)


if __name__ == "__main__":
    main()
