FROM rust:1.98.0-slim AS builder
WORKDIR /app

ENV SQLX_OFFLINE=true

COPY Cargo.toml Cargo.lock ./
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS final
WORKDIR /app
COPY --from=builder /app/target/release/guilds /app/guilds

EXPOSE 3000
CMD ["./guilds"]