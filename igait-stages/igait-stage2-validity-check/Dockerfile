FROM rust:1.93-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /app

# Copy lib and stage files
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage2-validity-check ./igait-stages/igait-stage2-validity-check

# Build the service
WORKDIR /app/igait-stages/igait-stage2-validity-check
RUN cargo build --release

# Runtime image
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    libgcc

# Copy binary from builder
COPY --from=builder /app/igait-stages/igait-stage2-validity-check/target/release/igait-stage2-validity-check /usr/local/bin/stage-service

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["stage-service"]

CMD ["stage-service"]
