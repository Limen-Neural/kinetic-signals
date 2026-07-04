# syntax=docker/dockerfile:1
FROM rust:1.88-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config=1.8.1-1 \
    libssl-dev=3.0.20-1~deb12u2 \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "" > src/lib.rs && \
    cargo build --release --all-features && \
    rm -rf src target/release/.fingerprint/kinetic[-_]signals* \
           target/release/deps/libkinetic[-_]signals* \
           target/release/deps/kinetic[-_]signals*

# Copy actual source
COPY src ./src
COPY examples ./examples
COPY tests ./tests

# Build with all features
RUN cargo build --release --all-features --examples

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3=3.0.20-1~deb12u2 \
    ca-certificates=20230311+deb12u1 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

# Copy demo binary
COPY --from=builder /app/target/release/examples/demo /usr/local/bin/demo

USER appuser

# Default command runs the demo example
CMD ["demo"]