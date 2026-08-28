# Accounts Service

---

## Getting Started

### 1. Prerequisites & Environment Setup

Make sure you are in the service directory (`apps/accounts`) and have a Python virtual environment configured:

```bash
# Create and activate virtual environment
python3 -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\Activate.ps1

# Install dependencies
(.venv) pip install -r requirements.txt
```

#### Set-up environment variables

```
ACCOUNTS_SERVICE_URL=localhost:3002
ACCOUNTS_SQL_CONNECTION_STRING="sqlite+aiosqlite:///./identity.db"
ACCOUNTS_REDIS_URL="redis://localhost:6379/0"
ACCOUNTS_JWT_TOKEN_EXPIRATION_MINUTES=15
ACCOUNTS_JWT_REFRESH_TOKEN_EXPIRATION_MINUTES=10080
ACCOUNTS_ENVIRONMENT=development | production
ACCOUNTS_JWT_PRIVATE_KEY_PATH=local_keys/jwt_private.pem
ACCOUNTS_JWT_PUBLIC_KEY_PATH=local_keys/jwt_public.pem
```

### 2. RSA Key Generation (RS256 JWT)

Before running the service for the first time, generate the RSA private and public key pair required for signing and verifying RS256 JWTs:
`python src/generate_keys.py`

This will create jwt_private.pem (used by Accounts to sign tokens) and jwt_public.pem (exposed over gRPC for the Gateway to verify tokens).

### 3. Protobuf Generation

Note: Due to a standard Python protoc import issue, generated gRPC files contain absolute imports that cause ModuleNotFoundError. Always run the patch script after generating stubs.

```
python post-generate-proto.py
```

### 4. Runing the Service

```
python src/python.py
```
