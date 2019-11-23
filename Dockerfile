FROM rust:latest

RUN apt-get update && \
  apt-get install -y cmake libssl-dev clang && \
  apt-get auto-remove && \
  apt-get auto-clean

RUN cargo install cargo-watch
RUN rustup component add clippy

WORKDIR /app
COPY . .
RUN cargo build

CMD cargo watch -x 'test -q'
