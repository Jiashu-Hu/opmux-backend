FROM rust:1.85-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY common/Cargo.toml common/Cargo.toml
COPY gateway/Cargo.toml gateway/Cargo.toml
COPY common/src common/src
COPY gateway/src gateway/src

RUN cargo build --release -p gateway

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/gateway /usr/local/bin/gateway

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000

EXPOSE 3000

CMD ["gateway"]
