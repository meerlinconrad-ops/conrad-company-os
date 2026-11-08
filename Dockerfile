# Multi-stage Docker build for ultra-compact Conrad CRM (~35MB total)
FROM rust:1.80-slim-bookworm as builder

WORKDIR /usr/src/conrad-crm
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Minimal runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates sqlite3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/conrad-crm/target/release/conrad-crm /app/conrad-crm

ENV PORT=8080
ENV DATABASE_URL=/data/crm.db
RUN mkdir -p /data

EXPOSE 8080
CMD ["/app/conrad-crm"]
