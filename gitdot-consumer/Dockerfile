# Build stage
FROM rust:1.92.0-bookworm AS builder

WORKDIR /app

RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev cmake && \
  rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release -p gitdot-consumer

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
  apt-get install -y ca-certificates && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gitdot-consumer /app/gitdot-consumer

CMD ["/app/gitdot-consumer"]
