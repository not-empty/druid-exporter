FROM rust:1.91.1-slim-bullseye AS build

WORKDIR /app

RUN cargo install cargo-watch --locked

COPY . .
