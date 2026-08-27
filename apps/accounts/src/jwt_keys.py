import os
from generate_keys import generate_keys;
_private_key_pem: bytes | None = None
_public_key_pem: bytes | None = None

def load_keys():
    global _private_key_pem, _public_key_pem
    
    priv_path = os.getenv("ACCOUNTS_JWT_PRIVATE_KEY_PATH", "jwt_private.pem")
    pub_path = os.getenv("ACCOUNTS_JWT_PUBLIC_KEY_PATH", "jwt_public.pem")
    env = os.getenv("ACCOUNTS_ENVIRONMENT", "development")

    if not os.path.exists(priv_path) or not os.path.exists(pub_path):
        if env == "production":
            raise RuntimeError(
                f"FATAL: RSA key files not found at '{priv_path}' or '{pub_path}'. "
                "Auto-generation is disabled in production!"
            )
        print(f"[{env.upper()}] RSA keys missing. Auto-generating for local development...")
        generate_keys(priv_path, pub_path)

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