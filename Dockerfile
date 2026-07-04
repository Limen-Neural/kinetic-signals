# syntax=docker/dockerfile:1
FROM rust:1.88-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "" > src/lib.rs && \
    cargo build --release --all-features && \
    rm -rf src

# Copy actual source
COPY src ./src
COPY examples ./examples
COPY tests ./tests

# Build with all features
RUN cargo build --release --all-features

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy demo binary
COPY --from=builder /app/target/release/examples/demo /usr/local/bin/demo

# Default command runs the demo example
CMD ["demo"]