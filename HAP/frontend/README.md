# HAP Frontend

Phoenix 1.7 LiveView frontend for the LongevityCommon subproject "HAP" (HepatoEmotions Project).

## Overview

This frontend provides a production-grade interface for visualizing and exploring the HAP project's concepts, parameters, and domain knowledge. It communicates with a Rust backend API to fetch structured data.

## Features

- Real-time dashboard with key metrics and correlations
- Detailed views for concepts, parameters, and domain knowledge
- Responsive design with Tailwind CSS
- Production-ready error handling and telemetry
- Graceful degradation when backend is unavailable

## Architecture

- **Framework**: Phoenix 1.7 with LiveView
- **Styling**: Tailwind CSS
- **HTTP Client**: Req with retry logic
- **Telemetry**: Built-in Phoenix telemetry with custom metrics

## Configuration

Environment variables:
- `BACKEND_URL`: URL of the Rust backend (default: http://localhost:3010)
- `PORT`: HTTP port for the frontend (default: 4010)
- `SECRET_KEY_BASE`: Phoenix secret key base

## Development

```bash
cd frontend
mix deps.get
mix assets.setup
mix phx.server
```

Visit `http://localhost:4010`

## Production

The application includes a Dockerfile for containerized deployment:

```bash
docker build -t hap-frontend .
docker run -p 4010:4010 -e BACKEND_URL=http://backend:3010 hap-frontend
```

## API Integration

The frontend expects the following backend endpoints:

- `GET /api/concept` - Returns structured CONCEPT data
- `GET /api/parameters` - Returns parameters list
- `GET /api/knowledge` - Returns domain knowledge concepts

## Telemetry

Custom telemetry events:
- `hap_frontend.backend_client.request` - HTTP request duration
- `hap_frontend.backend_client.error` - Error counts by type
- `hap_frontend.backend_client.measure` - Periodic measurements

## License

Part of the LongevityCommon project. See project root for licensing information.