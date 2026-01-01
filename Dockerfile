FROM rust:1.85 AS builder
WORKDIR /usr/src/antislop

# Copy source
COPY . .

# Build release binary
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/antislop/target/release/antislop /usr/local/bin/antislop
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["antislop"]
