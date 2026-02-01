FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage3-reframing ./igait-stages/igait-stage3-reframing

WORKDIR /app/igait-stages/igait-stage3-reframing
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ffmpeg ca-certificates libgcc
COPY --from=builder /app/igait-stages/igait-stage3-reframing/target/release/igait-stage3-reframing /usr/local/bin/worker
ENV RUST_LOG=info
CMD ["worker"]
