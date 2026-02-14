# CLAUDE.md - Torvi Project Guidelines

**CRITICAL: All code, including comments, must be in English.**

## Project Overview

Torvi is a Rust backend server for managing single-elimination tournament brackets for image-based idea competitions. Built with Rocket.rs, MongoDB, and AWS S3.

## Tech Stack

- **Language**: Rust (2021 edition)
- **Framework**: Rocket.rs 0.5.1
- **Database**: MongoDB 3.1.1
- **Storage**: AWS S3
- **Auth**: JWT (jsonwebtoken) + bcrypt
- **Testing**: mockall + Rust built-in test framework

## Project Structure

```
src/
├── main.rs                    # Entry point, mounts routes on /api/*
├── common/
│   ├── guards.rs              # JWT AuthenticatedUser guard
│   └── tests/
├── config/
│   ├── database.rs            # MongoDB connection (fairing)
│   ├── jwt.rs                 # JWT secret config (fairing)
│   ├── s3.rs                  # AWS S3 config (fairing)
│   └── tests/
└── modules/
    ├── auth/                  # Login + JWT token generation
    ├── users/                 # User registration
    ├── tournaments/           # Tournament brackets + voting
    ├── opponents/             # Tournament participants (ideas)
    └── images/                # Image upload + processing pipeline
```

Each module follows the pattern: `model.rs` / `service.rs` / `controller.rs` / `mod.rs` / `tests/`

## Architecture & Patterns

### Layered Architecture

Dependencies flow inward: **Controller -> Service -> Repository -> Model**

- **Controllers**: Rocket route handlers. Receive HTTP requests, delegate to services, return responses.
- **Services**: Business logic. Defined as traits with async methods for testability. Implementations use `#[async_trait]`.
- **Repositories**: Data access abstraction (trait + impl). Encapsulate MongoDB operations.
- **Models**: Domain entities with serde Serialize/Deserialize. DTOs for request/response shapes.

### Dependency Injection

- Use trait-based DI: define traits for services and repositories, inject implementations.
- Use `Rocket::State<T>` for shared dependencies (DB client, configs).
- Wrap shared state in `Arc<T>` for thread-safe sharing.
- In tests, inject mocks generated with `mockall` (`#[automock]` on traits).

### Request Guards

- Use `FromRequest` guards for auth, not fairings.
- `AuthenticatedUser` guard extracts and validates JWT from `Authorization: Bearer <token>` header.
- Protected endpoints receive the guard as a parameter.

## Coding Conventions

### Rust Style

- `snake_case` for functions, methods, variables, modules
- `PascalCase` for types (structs, enums, traits)
- `SCREAMING_SNAKE_CASE` for constants and statics
- Keep modules focused: one domain entity per module
- Use `mod.rs` to re-export public items, minimize public surface

### Error Handling

- Use `Result<T, E>` for all fallible operations
- Never use `.unwrap()` or `.expect()` in production code (only in tests)
- Use `?` operator for error propagation
- Implement custom `Error` enum with `thiserror` or manual impl
- Error types must implement Rocket's `Responder` trait for proper HTTP responses
- Add context to errors at each layer (controller, service, repository)

### Async

- Use `async fn` with `#[async_trait]` for trait methods
- Always poll MongoDB futures to completion; never drop them early
- Use `tokio` as the async runtime

### MongoDB

- Use `bson::oid::ObjectId` for all `_id` fields consistently (not Uuid)
- Use `#[serde(rename = "_id")]` for id fields
- Let MongoDB auto-generate `_id` with `Option<ObjectId>`
- Reuse the `Client` instance via managed state (never create per-request)
- Create indexes at application startup

### Security

- Hash passwords with `bcrypt` (cost factor 10-12), never store plaintext
- Store JWT secret in environment variables, never hardcode
- Set appropriate JWT expiration times
- Validate all user input at handler boundaries
- Never log sensitive data (passwords, tokens, PII)
- Use `#[serde(skip_serializing)]` on password fields

## Testing

### Unit Tests

- Place in the same file: `#[cfg(test)] mod tests { ... }`
- Mock dependencies with `mockall` (`#[automock]` on traits)
- Test one behavior per test function
- Test error paths, not just happy paths

### Integration Tests

- Place in `tests/` directory at crate root
- Use Rocket's `local::asynchronous::Client` for route testing
- Use a test database or mock MongoDB client

### Mocking

- Use `expect_method_name()` to configure mock behavior
- Set return values with `.returning(|args| result)`
- Use `.times(n)` only when testing interaction counts (retry mechanisms, etc.)
- Mock at repository/service boundaries, not internal details

## Environment Variables

```env
AWS_REGION=                # AWS region (e.g., us-east-1)
AWS_ACCESS_KEY_ID=         # AWS access key ID
AWS_SECRET_ACCESS_KEY=     # AWS secret access key
AWS_S3_BUCKET=             # S3 bucket name
JWT_SECRET=                # Secret key for JWT signing (min 256 bits)
MONGODB_URI=               # MongoDB connection string
MONGODB_NAME=              # MongoDB database name
```

## Common Commands

```bash
cargo build                # Build the project
cargo run                  # Run the server
cargo test                 # Run all tests
cargo test <module>        # Run tests for a specific module
cargo clippy               # Lint the code
cargo fmt                  # Format the code
```

## Git Workflow

### Branch Structure

- `main` - Production branch (always stable)
- `feature/xxx` - Branches for new features
- `hotfix/xxx` - Branches for urgent fixes

### Commit Conventions

**IMPORTANT**: Granular and descriptive commits.

- Each commit must be granular and contain a single responsibility
- Use gitmoji emoji before the message to indicate the type of change
- Descriptive message in English after the emoji
- Emoji reference: gitmoji.dev

### Commit Examples

```bash
# Add feature
git commit -m ":sparkles: add CreateWokuCommand handler"

# Fix bug
git commit -m ":bug: fix authentication token expiration"

# Improve structure/format
git commit -m ":art: improve folder structure in wokus module"

# Refactoring
git commit -m ":recycle: refactor user service to use CQRS"

# Add test
git commit -m ":white_check_mark: add unit tests for WokusService"

# Documentation
git commit -m ":memo: update API documentation"

# Add dependency
git commit -m ":heavy_plus_sign: add bullmq for queue processing"

# Remove dependency/code
git commit -m ":heavy_minus_sign: remove unused dependency"
```

### Common Gitmoji Reference

| Emoji              | Code                 | Usage                          |
| ------------------ | -------------------- | ------------------------------ |
| :sparkles:         | `:sparkles:`         | New feature                    |
| :bug:              | `:bug:`              | Bug fix                        |
| :art:              | `:art:`              | Structure/format improvement   |
| :recycle:          | `:recycle:`          | Refactoring                    |
| :white_check_mark: | `:white_check_mark:` | Add/update tests               |
| :memo:             | `:memo:`             | Documentation                  |
| :heavy_plus_sign:  | `:heavy_plus_sign:`  | Add dependency                 |
| :heavy_minus_sign: | `:heavy_minus_sign:` | Remove dependency              |
| :fire:             | `:fire:`             | Remove code/files              |
| :construction:     | `:construction:`     | Work in progress               |
| :lock:             | `:lock:`             | Security fix                   |
| :wrench:           | `:wrench:`           | Configuration change           |
| :tada:             | `:tada:`             | Initial commit / begin project |
| :rocket:           | `:rocket:`           | Deploy                         |
| :ambulance:        | `:ambulance:`        | Critical hotfix                |
