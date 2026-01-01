FROM rust:1.85 AS builder
WORKDIR /usr/src/antislop

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY crates ./crates
COPY src ./src

# Build release binary
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/antislop/target/release/antislop /usr/local/bin/antislop
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["antislop"]
