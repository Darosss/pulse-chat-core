# Pulse chat core

A real time chat backend built as a polyglot microservice system using gRPC and Protocol Buffers

## Getting started

### Quick Start with Docker (Recommended)

The entire microservice architecture (PostgreSQL, Redis, Python Accounts, C# Messages, and Rust Gateway) can be launched simultaneously using Docker Compose:

```bash
docker compose up --build
```

Once running, access the services via (default ports):

API Gateway (HTTP & WebSockets): http://localhost:3000

PostgreSQL: localhost:5432

Redis: localhost:6379

### Service startup:

- microservices & gateway located in `apps/` folder
- each service can be started individually during development:
  - gateway(**rust**): `cd apps/gateway && cargo run`
  - message(**c**#): `cd apps/message & dotnet watch` _or f5 in some IDE's_
  - pressence(**go**): `soon`
  - accounts/auth(**python**): check [apps/accounts/README](apps/accounts/README.md)
- proto folder contains all .proto files
- root package contains `npm run proto:generate` script to generate needed types/files for each language based on buf.gen.yaml file
- Soon more

## Estimated Architecture in mind

```text
                                   [ Client / Web Browser / Mobile ]
                                                 │
                                          (WebSockets / HTTP REST)
                                                 ▼
                     ┌────────────────────────────────────────────────────────────────┐
                     │                   RUST GATEWAY (Axum + Tokio)                  │
                     └──────┬──────────────┬──────────────┬──────────────┬────────────┘
                            │              │              │              │
                     (gRPC Unary)   (gRPC Unary)   (gRPC Stream)  (gRPC Unary)
                            │              │              │              │
                            ▼              ▼              ▼              ▼
                     ┌─────────────┐┌─────────────┐┌─────────────┐┌───────────────────┐
                     │  C#         ││    Go       ││    Python   ││  FASTIFY (NODE)   │
                     │  Messages   ││    Presence ││    Auth &   ││  Bots, Webhooks   │
                     │  & Guilds   ││    Engine   ││    Shop     ││  & Integrations   │
                     └──────┬──────┘└─────────────┘└─────────────┘└─────────┬─────────┘
                            │                                               │
                     (gRPC Stream)                                   (gRPC Unary)
                            ▼                                               ▼
                     ┌──────────────┐                             ┌───────────────────┐
                     │    C++       │                             │ External Services │
                     │    Audio     │                             │ (GitHub, Twitch,  │
                     │    Transcoder│                             │ Stripe, Webhooks) │
                     └──────────────┘                             └───────────────────┘
```
