FROM rust:slim
WORKDIR /app

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y \
  git

RUN rustup target add x86_64-unknown-linux-musl

COPY ./ ./
RUN cargo build

CMD cargo run
