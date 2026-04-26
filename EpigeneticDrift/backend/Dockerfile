# Build stage
FROM rust:1.70-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev postgresql-dev

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy source files for initial build caching
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src
COPY migrations ./migrations
COPY config ./config

# Build application
RUN cargo build --release

# Runtime stage
FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache libgcc openssl ca-certificates postgresql-client

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/epigeneticdrift-backend /app/

# Copy configuration and migrations
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/config ./config

# Create non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

# Set environment variables
ENV PORT=3007
ENV RUN_MODE=production
ENV DATABASE_URL="postgres://cn:cn@localhost/epigeneticdrift_db"

# Expose port
EXPOSE 3007

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3007/health || exit 1

# Run the application
CMD ["/app/epigeneticdrift-backend"]