import os

_private_key_pem: bytes | None = None
_public_key_pem: bytes | None = None

def load_keys():
    global _private_key_pem, _public_key_pem
    
    priv_path = os.getenv("JWT_PRIVATE_KEY_PATH", "jwt_private.pem")
    pub_path = os.getenv("JWT_PUBLIC_KEY_PATH", "jwt_public.pem")

    with open(priv_path, "rb") as f:
        _private_key_pem = f.read()

    with open(pub_path, "rb") as f:
        _public_key_pem = f.read()

def get_private_key() -> bytes:
    if not _private_key_pem:
        load_keys()
    return _private_key_pem

def get_public_key() -> bytes:
    if not _public_key_pem:
        load_keys()
    return _public_key_pem