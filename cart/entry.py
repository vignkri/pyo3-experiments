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
    df = pl.read_ipc(msg)

    # Operate on the dataframe
    print("Dataframe: %s" % df)

    data = "Hello".encode(ENCODING)
    return data
