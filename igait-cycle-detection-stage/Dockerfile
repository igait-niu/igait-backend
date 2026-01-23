# Multi-stage build for Stage 5: Cycle Detection
FROM rust:1.75-alpine as builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig

# Copy workspace files
COPY Cargo.toml ./
COPY igait-lib ./igait-lib
COPY igait-cycle-detection-stage ./igait-cycle-detection-stage

# Build the stage service
RUN cargo build --release -p igait-cycle-detection-stage

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    libgcc

# Copy the binary
COPY --from=builder /app/target/release/igait-cycle-detection-stage /usr/local/bin/stage-service

ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["stage-service"]
