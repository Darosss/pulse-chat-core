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
ACCOUNTS_SQL_CONNECTION_STRING ="sqlite+aiosqlite:///./identity.db",
ACCOUNTS_JWT_SECRET=
```

### 2. Protobuf Generation

Note: Due to a standard Python protoc import issue, generated gRPC files contain absolute imports that cause ModuleNotFoundError. Always run the patch script after generating stubs.

```
python post-generate-proto.py
```

### 3. Runing the Service

```
python src/python.py
```
