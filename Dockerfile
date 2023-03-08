FROM rust

WORKDIR /usr/src/app
COPY . .

ENTRYPOINT cargo test