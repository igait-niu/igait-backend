# syntax=docker/dockerfile:1

FROM rust:1.75-slim

WORKDIR /
COPY . .

RUN apt-get update
RUN apt-get install -y pkg-config curl libssl-dev openssl
RUN ["cargo", "build"]

VOLUME /data
VOLUME /.ssh

CMD ["/target/debug/igait-backend"]
EXPOSE 3000
