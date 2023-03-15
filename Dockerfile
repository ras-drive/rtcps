FROM rust

WORKDIR /usr/src/app
COPY .cargo/ .cargo/
COPY src src
COPY tests tests
COPY benches benches
COPY Cargo.toml Cargo.lock ./
COPY common_ports.csv common_ports.csv

ENTRYPOINT cargo test