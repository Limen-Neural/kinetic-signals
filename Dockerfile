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
    echo "fn main() {}" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source
COPY src ./src
COPY examples ./examples
COPY tests ./tests

# Build with all features
RUN cargo build --release --all-features

# Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Copy binaries (ignore if no examples exist)
COPY --from=builder /app/target/release/examples/ /usr/local/bin/

# Default command runs tests
CMD ["cargo", "test", "--all-features"]