FROM rust:1.58-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    git \
    curl \
    libssl-dev \
    pkg-config \
    mingw-w64 \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

RUN rustup target add `uname --machine`-unknown-linux-musl

COPY ./ ./
RUN cargo build --target `uname --machine`-unknown-linux-musl

# add developing tools
RUN cargo install cargo-outdated

CMD cargo run
