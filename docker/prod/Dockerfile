FROM rust:1.72.0-slim-bullseye AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:11-slim as prod

ARG USER=app
ENV HOME /home/$USER

WORKDIR /home/app

COPY --from=builder /app/target/release/druid-expo /usr/local/bin/druid-expo
COPY --from=builder /app/src/static /home/app/src/static

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    $USER \
    && chown $USER:$USER -R $HOME \
    && touch .env

USER $USER

ENTRYPOINT "/usr/local/bin/druid-expo"