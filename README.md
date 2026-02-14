<div align="center">

# Torvi

**A high-performance Rust backend for managing single-elimination tournament brackets with real-time WebSocket updates.**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Rocket](https://img.shields.io/badge/rocket-0.5.1-red.svg)](https://rocket.rs/)
[![MongoDB](https://img.shields.io/badge/mongodb-3.1-green.svg?logo=mongodb)](https://www.mongodb.com/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
[![Tests](https://img.shields.io/badge/tests-254%20passing-brightgreen.svg)]()

</div>

---

Torvi powers image-based idea competitions where participants submit visual concepts that compete head-to-head in a bracket-style elimination format. Built with Rocket.rs, it provides secure JWT authentication, anonymous voting, cursor-based pagination, and real-time tournament updates over WebSocket.

## Features

- **Tournament Brackets** - Full single-elimination bracket generation with automatic round progression
- **Real-time Updates** - WebSocket connections for live vote counts, match results, and tournament events
- **Dual Authentication** - JWT-based auth for registered users + anonymous tokens for guest voters
- **Invite System** - Generate invite codes with configurable max uses and expiration
- **Image Pipeline** - Upload, process (WebP conversion), and serve images via AWS S3
- **Cursor-based Pagination** - Efficient, scalable pagination for large datasets
- **Health Checks** - Liveness and readiness probes for container orchestration
- **Security Headers** - Rocket Shield with restrictive permissions policy

## Tech Stack

| Component | Technology | Version |
|-----------|-----------|---------|
| Language | Rust | 2021 edition |
| Web Framework | [Rocket.rs](https://rocket.rs/) | 0.5.1 |
| Database | [MongoDB](https://www.mongodb.com/) | 3.1.1 |
| WebSocket | [rocket_ws](https://crates.io/crates/rocket_ws) | 0.1.1 |
| Object Storage | [AWS S3](https://aws.amazon.com/s3/) | aws-sdk-s3 1.65 |
| Auth | [jsonwebtoken](https://crates.io/crates/jsonwebtoken) + [bcrypt](https://crates.io/crates/bcrypt) | 9.3 / 0.16 |
| Image Processing | [image](https://crates.io/crates/image) + [webp](https://crates.io/crates/webp) | 0.25 / 0.3 |
| Testing | [mockall](https://crates.io/crates/mockall) | 0.13 |

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.75+ (stable)
- [MongoDB](https://www.mongodb.com/docs/manual/installation/) 6.0+
- AWS account with S3 access

### Setup

```bash
git clone https://github.com/wokuApp/torvi.git
cd torvi

cp .env.example .env
# Edit .env with your credentials (see Configuration below)

cargo build
cargo run
```

The server starts at `http://0.0.0.0:8000`.

### Verify Installation

```bash
# Liveness check
curl http://localhost:8000/health/live

# Readiness check (verifies MongoDB connection)
curl http://localhost:8000/health/ready
```

## Configuration

### Environment Variables

Copy `.env.example` and configure:

```env
# MongoDB
MONGODB_URI=mongodb://localhost:27017
MONGODB_NAME=torvi

# JWT
JWT_SECRET=your-secret-key-min-256-bits

# AWS S3
AWS_REGION=us-east-1
AWS_ACCESS_KEY_ID=your-access-key-id
AWS_SECRET_ACCESS_KEY=your-secret-access-key
AWS_S3_BUCKET=your-bucket-name

# CORS (comma-separated origins)
CORS_ALLOWED_ORIGINS=http://localhost:3000

# Logging
RUST_LOG=info,torvi=debug
```

### Rocket Configuration

Server settings are in `Rocket.toml`:

| Setting | Default | Description |
|---------|---------|-------------|
| `address` | `0.0.0.0` | Bind address |
| `port` | `8000` | Listen port |
| `limits.json` | `2 MiB` | Max JSON body size |
| `limits.data-form` | `10 MiB` | Max form/upload size |
| `shutdown.grace` | `5s` | Graceful shutdown timeout |

## API Reference

### Authentication

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| `POST` | `/api/auth/register` | Register a new user | - |
| `POST` | `/api/auth/login` | Login, receive JWT tokens | - |
| `POST` | `/api/auth/refresh` | Refresh access token | - |
| `POST` | `/api/auth/anonymous` | Get anonymous voter token | - |

**Register:**
```http
POST /api/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "name": "John Doe",
  "password": "securepassword"
}
```

**Response:**
```json
{
  "access_token": "eyJhbG...",
  "refresh_token": "eyJhbG...",
  "token_type": "Bearer",
  "user": {
    "id": "507f1f77bcf86cd799439011",
    "email": "user@example.com",
    "name": "John Doe"
  }
}
```

### Users

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| `GET` | `/api/users/me` | Get current user profile | JWT |
| `GET` | `/api/users/:id` | Get public user profile | - |
| `PUT` | `/api/users/me` | Update current user | JWT |
| `DELETE` | `/api/users/me` | Delete current user | JWT |

### Tournaments

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| `POST` | `/api/tournaments/create` | Create tournament | JWT |
| `GET` | `/api/tournaments` | List my tournaments (paginated) | JWT |
| `GET` | `/api/tournaments/:id` | Get tournament details | Participant |
| `PUT` | `/api/tournaments/:id` | Update tournament | JWT (owner) |
| `DELETE` | `/api/tournaments/:id` | Delete tournament | JWT (owner) |
| `POST` | `/api/tournaments/:id/pause` | Pause tournament | JWT (owner) |
| `POST` | `/api/tournaments/:id/resume` | Resume tournament | JWT (owner) |
| `GET` | `/api/tournaments/:id/bracket` | Get bracket view | Participant |
| `GET` | `/api/tournaments/:id/results` | Get tournament results | - |
| `GET` | `/api/tournaments/:tid/matches/:mid` | Get match detail | Participant |
| `POST` | `/api/tournaments/match/vote` | Cast a vote | Participant |
| `POST` | `/api/tournaments/:id/invite` | Create invite link | JWT (owner) |
| `POST` | `/api/tournaments/:id/join` | Join via invite code | - |

**Pagination** uses cursor-based approach:
```http
GET /api/tournaments?limit=20&cursor=507f1f77bcf86cd799439011
```

### Opponents (Ideas)

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| `POST` | `/api/opponents/create` | Create opponent | JWT |
| `GET` | `/api/opponents` | List my opponents (paginated) | JWT |
| `GET` | `/api/opponents/:id` | Get opponent detail | JWT |
| `PUT` | `/api/opponents/:id` | Update opponent | JWT (owner) |
| `DELETE` | `/api/opponents/:id` | Delete opponent | JWT (owner) |

### Images

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| `POST` | `/api/images/upload` | Upload image (max 10MB) | JWT |
| `GET` | `/api/images/:id` | Get image metadata | JWT |
| `DELETE` | `/api/images/:id` | Delete image | JWT (owner) |

Images are uploaded as raw binary with `Content-Type: image/*` header.

### WebSocket

Connect to receive real-time tournament events:

```
ws://localhost:8000/ws/tournaments/<tournament_id>?token=<jwt_or_anonymous_token>
```

**Server Events:**

| Event | Description |
|-------|-------------|
| `vote_cast` | A vote was submitted (includes current counts) |
| `match_completed` | A match has a winner |
| `round_completed` | All matches in a round finished |
| `tournament_completed` | Tournament has a final winner |
| `participant_joined` | New participant joined |
| `tournament_paused` | Tournament was paused by owner |
| `tournament_resumed` | Tournament was resumed by owner |

**Client Messages:**

| Message | Description |
|---------|-------------|
| `{"type": "ping"}` | Heartbeat ping (server responds with pong) |

### Health Checks

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health/live` | Liveness probe (always `200 OK`) |
| `GET` | `/health/ready` | Readiness probe (checks MongoDB connection) |

### Error Responses

All errors follow a consistent format:

```json
{
  "error": "Bad Request",
  "message": "Bad request: Invalid tournament ID"
}
```

| Status Code | Error Type |
|-------------|-----------|
| `400` | Bad Request / Validation Error |
| `401` | Unauthorized |
| `403` | Forbidden |
| `404` | Not Found |
| `500` | Internal Server Error / Database Error |

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      Rocket.rs                          │
│  ┌───────────┐  ┌──────────┐  ┌──────────────────────┐ │
│  │ Security  │  │  Guards   │  │  Fairings (DI)       │ │
│  │  Shield   │  │ JWT Auth  │  │  DB, S3, JWT, Svc    │ │
│  └───────────┘  └──────────┘  └──────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│                    Controllers                          │
│  auth │ users │ tournaments │ opponents │ images │ ws   │
├─────────────────────────────────────────────────────────┤
│                     Services                            │
│        (trait-based with Arc<dyn T + Send + Sync>)      │
├─────────────────────────────────────────────────────────┤
│                    Repositories                         │
│               (MongoDB collection access)               │
├─────────────────────────────────────────────────────────┤
│                      Models                             │
│         (Domain entities + DTOs with serde)              │
└─────────────────────────────────────────────────────────┘
         │                              │
    ┌────▼────┐                  ┌──────▼──────┐
    │ MongoDB │                  │   AWS S3    │
    └─────────┘                  └─────────────┘
```

### Project Structure

```
src/
├── main.rs                          # Entry point, mounts all routes
├── error.rs                         # Unified error types with Responder
├── common/
│   ├── guards.rs                    # AuthenticatedUser, TournamentParticipant
│   └── pagination.rs                # Cursor-based pagination
├── config/
│   ├── database.rs                  # MongoDB connection (fairing)
│   ├── jwt.rs                       # JWT secret config (fairing)
│   ├── s3.rs                        # AWS S3 config (fairing)
│   ├── security.rs                  # Shield headers (fairing)
│   ├── indices.rs                   # MongoDB index creation
│   └── services.rs                  # Dependency injection wiring
└── modules/
    ├── auth/                        # Login, register, token refresh, anonymous tokens
    ├── users/                       # User CRUD operations
    ├── tournaments/                 # Tournament lifecycle, brackets, voting, invites
    ├── opponents/                   # Tournament participants (ideas)
    ├── images/                      # Image upload, processing, S3 storage
    ├── health/                      # Liveness & readiness probes
    └── websocket/
        ├── broadcaster.rs           # TournamentBroadcaster (DashMap + tokio broadcast)
        ├── controller.rs            # WS route handler with auth
        └── model.rs                 # Event & message types
```

### Key Design Decisions

- **Trait-based DI**: All services are defined as traits and injected via `Arc<dyn Service + Send + Sync>`, enabling full mockability in tests
- **Dual Auth Model**: `VoterId` enum supports both `Registered(ObjectId)` and `Anonymous(String)` voters in the same tournament
- **Request Guards**: `AuthenticatedUser` (JWT required) and `TournamentParticipant` (JWT or anonymous token) for granular access control
- **Fire-and-forget Broadcasting**: WebSocket events are dispatched via `TournamentBroadcaster` without blocking the HTTP response
- **Cursor-based Pagination**: Uses MongoDB `ObjectId` as cursor for stable, performant pagination across large collections

## Development

### Running Tests

```bash
# Run all unit tests (no MongoDB required)
cargo test

# Run a specific module's tests
cargo test tournaments
cargo test auth

# Run with output
cargo test -- --nocapture

# Run ignored tests (requires running MongoDB)
cargo test -- --ignored
```

> Currently: **254 tests passing**, 40 ignored (require MongoDB instance)

### Code Quality

```bash
# Lint
cargo clippy -- -D warnings

# Format
cargo fmt

# Check formatting
cargo fmt -- --check
```

### TDD Workflow

This project strictly follows Test-Driven Development:

1. **Red** - Write a failing test that defines expected behavior
2. **Green** - Write minimum code to make the test pass
3. **Refactor** - Clean up while keeping all tests green

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch from `develop` (`git checkout -b feature/your-feature`)
3. Follow TDD - write tests before implementation
4. Use [gitmoji](https://gitmoji.dev/) commit conventions:
   ```bash
   git commit -m ":sparkles: add tournament filtering"
   git commit -m ":bug: fix vote counting edge case"
   git commit -m ":white_check_mark: add bracket generation tests"
   ```
5. Push and open a Pull Request against `develop`

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Author

**Diego Orrego** - CTO at [Woku](https://woku.app)

- GitHub: [@diorrego](https://github.com/diorrego)
- Email: diego@woku.app

---

<div align="center">

Built with [Rocket.rs](https://rocket.rs/) by [Woku](https://woku.app)

</div>
