FROM rust:1.76 as builder
WORKDIR /usr/src/antislop
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/antislop/target/release/antislop /usr/local/bin/antislop
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["antislop"]
