FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Copy Cargo manifests for dependency caching
COPY igait-lib/Cargo.toml ./igait-lib/Cargo.toml
COPY igait-stages/igait-stage6-prediction/Cargo.toml ./igait-stages/igait-stage6-prediction/Cargo.toml

# Create dummy source files to cache dependencies
RUN mkdir -p igait-lib/src && echo "pub fn dummy() {}" > igait-lib/src/lib.rs
RUN mkdir -p igait-stages/igait-stage6-prediction/src && echo "fn main() {}" > igait-stages/igait-stage6-prediction/src/main.rs

# Build dependencies only (this layer will be cached)
WORKDIR /app/igait-stages/igait-stage6-prediction
RUN cargo build --release 2>/dev/null || true

# Now copy actual source files
WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage6-prediction ./igait-stages/igait-stage6-prediction

# Build with actual source
WORKDIR /app/igait-stages/igait-stage6-prediction
# Clean the dummy build artifacts first
RUN cargo clean --release
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates python3 python3-pip \
    && pip3 install --no-cache-dir --break-system-packages tensorflow numpy scikit-learn \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/igait-stages/igait-stage6-prediction/target/release/igait-stage6-prediction /usr/local/bin/worker
# COPY models/ /app/models/  # Add trained ML model when ready
ENV RUST_LOG=info
CMD ["worker"]
