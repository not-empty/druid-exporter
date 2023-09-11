FROM rust:1.72.0-slim-bullseye AS build

WORKDIR /app

RUN cargo install cargo-watch

COPY . .