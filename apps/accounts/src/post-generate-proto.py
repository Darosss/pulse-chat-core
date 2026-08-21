
import os
import sys

if __name__ == "__main__":
        
    GRPC_FILE = os.path.join(os.path.dirname(__file__), "pb", "auth_pb2_grpc.py")

    if os.path.exists(GRPC_FILE):
        with open(GRPC_FILE, "r", encoding="utf-8") as f:
            content = f.read()

        if "import auth_pb2 as" in content:
            patched = content.replace("import auth_pb2 as", "from . import auth_pb2 as")
            with open(GRPC_FILE, "w", encoding="utf-8") as f:
                f.write(patched)