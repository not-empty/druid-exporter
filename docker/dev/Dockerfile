FROM rust:1.77.0-slim-bullseye AS build

WORKDIR /app

RUN cargo install cargo-watch

COPY . .