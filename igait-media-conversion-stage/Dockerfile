FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY igait-lib ./igait-lib
COPY igait-stages/stage1-media-conversion ./igait-stages/stage1-media-conversion

# Build the service
RUN cargo build --release -p igait-stage1-media-conversion

# Runtime image
FROM alpine:latest

# Install FFmpeg for video conversion
RUN apk add --no-cache ffmpeg ca-certificates

# Copy binary from builder
COPY --from=builder /app/target/release/igait-stage1-media-conversion /usr/local/bin/stage-service

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["stage-service"]
