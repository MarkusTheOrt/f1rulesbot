FROM rust:latest as build-stage

WORKDIR /app

ARG DATABASE_URL

COPY . .

RUN rustup toolchain install nightly
RUN cargo +nightly install --path .


FROM debian:buster-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=build-stage /usr/local/cargo/bin/f1rulesbot /usr/local/bin/f1rulesbot
CMD ["f1rulesbot"]