FROM ubuntu:22.04
FROM python:3.12.1
FROM rust:latest

WORKDIR /

COPY . .

CMD ["cargo", "run"]

EXPOSE 3000
