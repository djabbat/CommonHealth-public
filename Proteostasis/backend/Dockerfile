# Builder stage
FROM rust:1.70-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/proteostasis

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 proteostasis

USER proteostasis

WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/proteostasis/target/release/proteostasis_backend /app/

# Copy migrations
COPY --from=builder /usr/src/proteostasis/migrations ./migrations

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["curl", "-f", "http://localhost:3008/health"] || exit 1

# Expose port
EXPOSE 3008

# Run the application
CMD ["./proteostasis_backend"]