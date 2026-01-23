FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig

WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY igait-lib ./igait-lib
COPY igait-media-conversion-stage ./igait-media-conversion-stage

# Build the service
RUN cargo build --release -p igait-media-conversion-stage

# Runtime image
FROM alpine:latest

# Install FFmpeg for video conversion
RUN apk add --no-cache \
    ffmpeg \
    ca-certificates \
    libgcc

# Copy binary from builder
COPY --from=builder /app/target/release/igait-media-conversion-stage /usr/local/bin/stage-service

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["stage-service"]
