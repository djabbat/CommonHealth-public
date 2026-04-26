FROM elixir:1.15-alpine AS build

# Install build dependencies
RUN apk add --no-cache build-base npm git python3 curl

# Prepare working directory
WORKDIR /app

# Install Hex and Rebar
RUN mix local.hex --force && \
    mix local.rebar --force

# Copy mix files
COPY mix.exs mix.lock ./

# Install dependencies
ENV MIX_ENV=prod
RUN mix deps.get --only $MIX_ENV
RUN mix deps.compile

# Copy assets
COPY assets assets
RUN npm install --prefix assets
RUN mix assets.deploy

# Copy application code
COPY lib lib
COPY config config

# Compile application
RUN mix compile

# Build release
RUN mix release

# Production stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache openssl ncurses-libs

WORKDIR /app

# Copy release from build stage
COPY --from=build /app/_build/prod/rel/biosense_frontend ./

# Create non-root user
RUN adduser -S -h /app -u 1000 app
USER app

# Set environment
ENV HOME=/app
ENV MIX_ENV=prod
ENV PORT=4004
ENV BACKEND_URL=http://localhost:3004

# Expose port
EXPOSE 4004

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:4004/ || exit 1

# Run application
CMD