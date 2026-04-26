FROM rust:1.70-slim as builder

WORKDIR /usr/src/biosense_backend

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build for release
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/biosense_backend/target/release/biosense_backend /usr/local/bin/

# Copy migrations
COPY --from=builder /usr/src/biosense_backend/migrations ./migrations

# Create non-root user
RUN useradd -m -u 1000 biosense
USER biosense

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3004/health || exit 1

EXPOSE 3004

CMD ["biosense_backend"]