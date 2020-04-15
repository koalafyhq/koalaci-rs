FROM rust:1.40.0 AS builder

WORKDIR /app/koalaci

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new koalaci

COPY Cargo.toml Cargo.lock ./
COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM docker:stable

RUN apk add --no-cache bash

COPY --from=builder /app/koalaci/target/x86_64-unknown-linux-musl/release/koalaci .
COPY --from=builder /app/koalaci/ci/ ./ci

USER 1000

CMD ["./koalaci"]
