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
  - pressence(**go**): `soon` - for now it depends on gateway
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

### Estimated Roadmap

#### Core

- [x] **Initial core:** gateway(rust), (accounts)Python, message(C#)
- [x] **Docker Setup:** docker-compose configs and logic
- [x] **Auth Pipeline:** JWT generation, token validation, and path/query extraction
- [ ] **Presence (Go):** Online/offline status tracking:
  - _**for now it's all in gateway**_
- [ ] **Gateway/auth Caching:** Redis JWT validation caching at gateway layer

#### Messaging

- [x] **Chat Pipeline:** base chatroom hhstory, message creation, and stream handlers
- [x] **Room Security:** access check verification before socket upgrade
- [ ] **Private Chats:** 1-on-1 direct messages
- [ ] **Chat Extras:** typing indicators, message edits
- [ ] **Multi-Server Sync:** redis ws scaling

### Servers & Guilds

- [ ] **Guilds:** server creation
- [ ] **Text Channels:** multiple rooms per server
- [ ] **Notifications:** fastify webhook alerts

### Voice & Media

- [ ] **Music streaming:** audio playback and queue management
- [ ] **File Sharing:** image and file uploads
- [ ] **Voice Chat:** WebRTC real-time audio
