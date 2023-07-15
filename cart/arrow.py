#!/.venv/bin/python

import pyarrow as pa

def handler(msg: bytes) -> bytes:
    """Get a specific output from the system"""

    print("'%s' from Rust"  % msg.decode("utf-8"))
    data = "Hello".encode()
    return data
