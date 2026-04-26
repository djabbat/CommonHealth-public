# Epigenetic Drift Frontend

Phoenix 1.7 LiveView frontend for LongevityCommon subproject "Epigenetic Drift" (MCOA Counter #4).

## Overview

This frontend visualizes and interacts with the Epigenetic Drift counter data within the Multi-Counter Architecture of Organismal Aging (MCOA). It implements a production-quality Phoenix 1.7 application with LiveView, Tailwind CSS, and telemetry.

## Features

- **Dashboard**: Overview of all epigenetic drift entities with summary metrics
- **Detail View**: Detailed examination of individual counter instances
- **Counter Registry**: UI for managing MCOA counter definitions (per CORRECTIONS canon)
- **Sobol Sensitivity**: Visualization of parameter sensitivity analysis
- **HSC Lineage Tracking**: Visualization of hematopoietic stem cell lineage data
- **Production-ready**: Telemetry, error handling, graceful degradation

## Architecture

- **Phoenix 1.7+** with LiveView for real-time updates
- **Tailwind CSS** for styling
- **Req/HTTPoison** for backend communication
- **Telemetry** with metrics and monitoring
- **Sentry** integration for error tracking

## Configuration

Set environment variables:

```bash
export PORT=4007
export BACKEND_URL=http://localhost:3007
export SECRET_KEY_BASE="your-secret-key-base"
export PHX_HOST=localhost
export SENTRY_DSN="your-sentry-dsn"  # optional
```

## Installation

1. Clone the repository
2. Install dependencies: `mix deps.get`
3. Install Node.js dependencies: `npm install --prefix assets`
4. Start the server: `mix phx.server`

Or with Docker:

```bash
docker build -t epigeneticdrift-frontend .
docker run -p 4007:4007 -e BACKEND_URL=http://host.docker.internal:3007 epigeneticdrift-frontend
```

## Development

- Run `mix setup` to install dependencies
- Run `mix phx.server` for development server
- Visit `http://localhost:4007`

## Testing

- Run `mix test` for unit tests
- Run `mix credo` for code analysis
- Run `mix dialyzer` for type checking

##