# syntax=docker/dockerfile:1

# [ Arguments ]
ARG SSH_KEY
ENV SSH_KEY=$SSH_KEY


# [ Layer 1 ] Build the Rust crate as a layer
FROM rust:1.75-slim as build

WORKDIR /
COPY . .

RUN apt-get update
RUN apt-get install -y pkg-config curl libssl-dev openssl
RUN ["cargo", "build"]


# [ Layer 2 ] Production layer with SSH keys copied
FROM alpine

RUN apk add --no-cache openssh-client

COPY --from=build /target/debug/igait-backend /igait-backend
VOLUME /data

CMD ["/igait-backend"]
EXPOSE 3000
