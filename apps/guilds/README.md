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

### Local Development Setup

When running locally without Docker, SQLx checks queries at compile time against a live PostgreSQL instance

1. **Install SQLx CLI** (if not already installed):

```bash
    cargo install sqlx-cli --no-default-features --features postgres
```

2. Create Database & Run Migrations:

```bash
    cargo sqlx database create
    cargo sqlx migrate run
```

3. Update Offline Cache (run whenever you change SQL queries):

```bash
    cargo sqlx prepare
```

### 4. Runing the Service

```rs
// run in development mode
cargo run

//or run in release mode
cargo run --release
```
