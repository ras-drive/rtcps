FROM rust:alpine

WORKDIR /usr/src/app
COPY src src
COPY tests tests
COPY benches benches
COPY Cargo.toml Cargo.lock ./

ENTRYPOINT cargo test