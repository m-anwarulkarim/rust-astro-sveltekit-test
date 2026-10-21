# Multi-stage Dockerfile for Rust Actix-web Backend
FROM rust:1.82-bookworm AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src target/release/deps/shoes_backend* target/release/shoes-backend*

# Copy actual source code
COPY src ./src
COPY migrations ./migrations

# Build the real binary
RUN cargo build --release

# Minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/shoes-backend /app/shoes-backend

ENV PORT=8080
ENV RUST_LOG=info
EXPOSE 8080

CMD ["/app/shoes-backend"]
