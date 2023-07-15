#!/.venv/bin/python

import pyarrow as pa
import polars as pl

# static configurations for the operations
ENCODING: str = "utf-8"

def handler(msg: bytes) -> bytes:
    """Get a specific output from the system"""

    # note! this reads as a file instead of a running stream which is causing
    # the difference in bytes and correspondingly the overal operational
    # error across the rust-python divide
    ipc_stream_table = pa.ipc.open_stream(msg).read_all()

    # Generate table and process the results
    df = pl.from_arrow(ipc_stream_table)
    dataset_description = df.describe()
    print("Provided Description: %s" % dataset_description)

    data = "Hello".encode(ENCODING)
    return data
