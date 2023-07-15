#!/.venv/bin/python

import pyarrow as pa

# static configurations for the operations
ENCODING: str = "utf-8"

def handler(msg: bytes) -> bytes:
    """Get a specific output from the system"""

    print("'%s' from Rust"  % msg.decode(ENCODING))
    data = "Hello".encode(ENCODING)
    return data
