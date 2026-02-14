# Stage 1: Chef - prepare dependency recipe
FROM rust:1.83-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Stage 2: Planner - compute dependency recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Builder - build dependencies then application
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# Stage 4: Runtime - minimal production image
FROM debian:bookworm-slim AS runtime
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*
RUN useradd --create-home appuser
USER appuser
WORKDIR /app
COPY --from=builder /app/target/release/torvi /app/torvi
COPY --from=builder /app/Rocket.toml /app/Rocket.toml

ENV ROCKET_PROFILE=release
EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8000/health/live || exit 1

ENTRYPOINT ["/app/torvi"]
