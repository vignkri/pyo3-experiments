#!/.venv/bin/python

import pyarrow as pa

# static configurations for the operations
ENCODING: str = "utf-8"

def handler(msg: bytes) -> bytes:
    """Get a specific output from the system"""

    with pa.ipc.open_stream(msg) as reader:
        schema = reader.schema
        batches = [b for b in reader]
        print("Received schema: %s" % schema)

    data = "Hello".encode(ENCODING)
    return data
