FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage2-validity-check ./igait-stages/igait-stage2-validity-check

WORKDIR /app/igait-stages/igait-stage2-validity-check
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates libgcc
COPY --from=builder /app/igait-stages/igait-stage2-validity-check/target/release/igait-stage2-validity-check /usr/local/bin/worker
ENV RUST_LOG=info
CMD ["worker"]
