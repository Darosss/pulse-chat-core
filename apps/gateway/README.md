# Accounts Service

---

## Getting Started

### 1. Prerequisites & Environment Setup

Make sure you are in the service directory (`apps/gateway`) and have the Rust toolchain installed (`rustc` & `cargo`)

#### 2. Set-up environment variables

_create `.env` file or export hese vars in your shell environment_

```
PORT=3000
MESSAGE_SERVICE_URL=http://localhost:3001
ACCOUNTS_SERVICE_URL=http://localhost:3002
REDIS_URL=redis://localhost:6379
```

### 3. Runing the Service

```rs
// run in development mode
cargo run

//or run in release mode
cargo run --release
```
