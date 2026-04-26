# BioSense Backend

Measurement layer backend for the LongevityCommon MCOA framework. Handles raw biosignal data storage from wearable devices (EEG, HRV, Olfaction sensors).

## Architecture

- **Framework**: Axum 0.7 + Tokio runtime
- **Database**: PostgreSQL with SQLx
- **Validation**: Validator crate for request validation
- **Error Handling**: Custom AppError with proper HTTP status codes
- **Tracing**: Structured logging with tracing/tracing-subscriber

## Entities

1. **Device**: Wearable hardware with serial numbers and firmware versions
2. **Session**: Measurement sessions linking subject, device, and protocol
3. **EEG Measurement**: Raw EEG channel data with metadata
4. **HRV Measurement**: RR interval time series
5. **Olfaction Measurement**: Smell sensor readings

## API Endpoints

### Devices
- `GET /api/devices` - List devices
- `POST /api/devices` - Create device
- `GET /api/devices/:id` - Get device
- `PUT /api/devices/:id` - Update device
- `DELETE /api/devices/:id` - Delete device

### EEG Measurements
- `GET /api/eeg_measurements` - List EEG measurements
- `POST /api/eeg_measurements` - Upload raw EEG data
- `GET /api/eeg_measurements/:id` - Get EEG measurement

### HRV Measurements
- `GET /api/hrv_measurements` - List HRV measurements
- `POST /api/hrv_measurements` - Upload RR intervals
- `GET /api/hrv_measurements/:id` - Get HRV measurement

### Olfaction Measurements
- `GET /api/olfaction_measurements` - List olfaction measurements
- `POST /api/olfaction_measurements` - Upload sensor readings
- `GET /api/olfaction_measurements/:id` - Get olfaction measurement

### Sessions
- `GET /api/sessions` - List sessions
- `POST /api/sessions` - Create session
- `GET /api/sessions/:id` - Get session
- `PUT /api/sessions/:id` - Update session
- `DELETE /api/sessions/:id` - Delete session

## Important Design Decisions

1. **No χ_Ze computation**: Per CORRECTIONS_2026-04-22, the server only stores raw data. Ze analysis is performed client-side or in separate processing pipelines.

2. **Raw data storage**: EEG data stored as JSONB arrays (channel × samples). HRV as PostgreSQL float arrays.

3. **Subject anonymity**: Subject IDs are optional strings, allowing de-identified data collection.

4. **Session context**: All measurements can be linked to a session for protocol/environment tracking.

5. **Hardware parameters**: Device-specific configuration stored as JSONB for flexibility.

## Development

### Prerequisites
- Rust 1.70+
- PostgreSQL 14+
- Cargo

### Environment Setup
```bash
cp .env.example .env
# Edit .env with your database credentials
```

### Database Setup
```bash
createdb biosense_db
psql -d biosense_db -f migrations/001_initial.sql
```

### Running
```bash
cargo run
# Server runs on http://localhost:3004
```

### Testing
```bash
cargo test
```

## Deployment

### Docker
```bash
docker build -t biosense-backend .
docker run -p 3004:3004 --env-file .env biosense-backend
```

### Production Notes
1. Set `APP_ENV=production` in environment
2. Use proper PostgreSQL connection pooling
3. Configure CORS for your frontend domain
4. Enable HTTPS/TLS termination
5. Set up database backups

## MCOA Integration

This backend serves as the measurement layer for MCOA counters:
- **Counter S (χ_Ze)**: Raw EEG/HRV data for client-side Ze analysis
- **Counter A (autonomic)**: RR intervals for SDNN/RMSSD computation
- **Environmental data**: Olfaction sensor readings for VOC analysis

All computation happens downstream - this service focuses on reliable, timestamped raw data storage.