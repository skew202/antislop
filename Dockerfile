# Build stage
FROM rust:1.83 AS builder
WORKDIR /usr/src/antislop

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY crates ./crates
COPY src ./src
COPY benches ./benches
COPY config ./config
COPY data ./data

# Build release binary
RUN cargo build --release && strip /usr/src/antislop/target/release/antislop

# Runtime stage - minimal attack surface
FROM debian:bookworm-slim AS runtime

# Install only runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* /usr/share/doc/* /usr/share/man/*

# Create non-root user for security
RUN useradd -m -u 1000 -s /usr/sbin/nologin -c "AntiSlop user" antislop

# Copy binary from builder and set permissions
COPY --from=builder --chown=antislop:antislop /usr/src/antislop/target/release/antislop /usr/local/bin/antislop

# Set security-focused filesystem options
RUN chmod 755 /usr/local/bin/antislop && \
    mkdir -p /data && \
    chown -R antislop:antislop /data

# Switch to non-root user
USER antislop

# Set working directory
WORKDIR /data

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD antislop --version || exit 1

# Run as non-root with minimal privileges
ENTRYPOINT ["antislop"]
