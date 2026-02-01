FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage1-media-conversion ./igait-stages/igait-stage1-media-conversion

WORKDIR /app/igait-stages/igait-stage1-media-conversion
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ffmpeg ca-certificates libgcc
COPY --from=builder /app/igait-stages/igait-stage1-media-conversion/target/release/igait-stage1-media-conversion /usr/local/bin/worker
ENV RUST_LOG=info
CMD ["worker"]
