FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Copy Cargo manifests for dependency caching
COPY igait-lib/Cargo.toml ./igait-lib/Cargo.toml
COPY igait-stages/igait-stage3-reframing/Cargo.toml ./igait-stages/igait-stage3-reframing/Cargo.toml

# Create dummy source files to cache dependencies
RUN mkdir -p igait-lib/src && echo "pub fn dummy() {}" > igait-lib/src/lib.rs
RUN mkdir -p igait-stages/igait-stage3-reframing/src && echo "fn main() {}" > igait-stages/igait-stage3-reframing/src/main.rs

# Build dependencies only (this layer will be cached)
WORKDIR /app/igait-stages/igait-stage3-reframing
RUN cargo build --release 2>/dev/null || true

# Now copy actual source files
WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage3-reframing ./igait-stages/igait-stage3-reframing

# Build with actual source
WORKDIR /app/igait-stages/igait-stage3-reframing
# Clean the dummy build artifacts first
RUN cargo clean --release
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates libgcc
COPY --from=builder /app/igait-stages/igait-stage3-reframing/target/release/igait-stage3-reframing /usr/local/bin/worker
ENV RUST_LOG=info
CMD ["worker"]
