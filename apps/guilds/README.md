# Guilds Service

---

## Getting Started

### 1. Prerequisites & Environment Setup

Make sure you are in the service directory (`apps/guilds`) and have the Rust toolchain installed (`rustc` & `cargo`)

#### 2. Set-up environment variables

_create `.env` file or export hese vars in your shell environment_

```
GUILDS_SERVICE_URL=127.0.0.1:3003
DATABASE_URL=postgres://postgres:password@localhost:5432/guilds_db
```

### 3. Runing the Service

```rs
// run in development mode
cargo run

//or run in release mode
cargo run --release
```
