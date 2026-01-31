# Multi-stage build for igait-backend
FROM rust:1.93-slim AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy lib and backend
COPY igait-lib ./igait-lib
COPY igait-backend ./igait-backend

# Build the backend
WORKDIR /app/igait-backend
RUN cargo build --release -p igait-backend

# Runtime stage - use matching Debian version
FROM debian:trixie-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3t64 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/igait-backend/target/release/igait-backend /usr/local/bin/igait-backend

# Set environment
ENV RUST_LOG=info
ENV PORT=3000

EXPOSE 3000

CMD ["igait-backend"]
