# docker\worker.Dockerfile
FROM rust:1.94-bookworm AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src
COPY tests ./tests
COPY conf ./conf

RUN cargo build --release --bin worker

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/worker /usr/local/bin/worker
COPY conf ./conf
COPY data ./data

CMD ["worker"]
