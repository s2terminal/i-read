FROM rust:1.58-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    git \
    curl \
    libssl-dev \
    pkg-config \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

COPY ./ ./
RUN cargo build

# add developing tools
RUN cargo install cargo-outdated

CMD cargo run
