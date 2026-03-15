# docker\runner.Dockerfile
FROM rust:1.94-bookworm AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src
COPY tests ./tests
COPY conf ./conf

RUN cargo build --release --bin runner_api

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/runner_api /usr/local/bin/runner_api
COPY conf ./conf
COPY data ./data

EXPOSE 8080

CMD ["runner_api"]
