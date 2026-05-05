# syntax=docker/dockerfile:1.7

# ---- chef base ----
# cargo-chef separates dependency compilation into its own cached layer so
# `docker build` only recompiles deps when Cargo.toml/Cargo.lock changes.
FROM rust:1.92.0-bookworm AS chef
WORKDIR /app
RUN cargo install cargo-chef --locked

# ---- planner: compute the recipe (cheap, just inspects Cargo.toml's) ----
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- builder ----
FROM chef AS builder

# Install build dependencies (cmake/perl needed for rdkafka + vendored openssl)
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev cmake perl && \
  rm -rf /var/lib/apt/lists/*

# 1. Bake all dependencies into a separate cached layer.
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json -p gitdot-consumer

# 2. Copy real source and build only workspace crates.
COPY . .
RUN cargo build --release -p gitdot-consumer

# ---- runtime ----
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
  apt-get install -y ca-certificates && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gitdot-consumer /app/gitdot-consumer

CMD ["/app/gitdot-consumer"]
