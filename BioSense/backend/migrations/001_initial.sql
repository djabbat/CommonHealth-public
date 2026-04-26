-- BioSense backend initial migration
-- Creates all tables for the measurement layer

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Devices table (wearable hardware)
CREATE TABLE devices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    serial_number VARCHAR(50) NOT NULL UNIQUE,
    device_type VARCHAR(50) NOT NULL,
    firmware_version VARCHAR(50) NOT NULL,
    hardware_parameters JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_devices_serial_number ON devices(serial_number);
CREATE INDEX idx_devices_created_at ON devices(created_at);

-- Sessions table (measurement sessions)
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subject_id VARCHAR(100),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    protocol_type VARCHAR(50) NOT NULL,
    environment_conditions JSONB DEFAULT '{}'::jsonb,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sessions_subject_id ON sessions(subject_id);
CREATE INDEX idx_sessions_device_id ON sessions(device_id);
CREATE INDEX idx_sessions_created_at ON sessions(created_at);

-- EEG measurements table (raw EEG data)
CREATE TABLE eeg_measurements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    session_id UUID REFERENCES sessions(id) ON DELETE SET NULL,
    subject_id VARCHAR(100),
    recording_started_at TIMESTAMPTZ NOT NULL,
    recording_ended_at TIMESTAMPTZ NOT NULL,
    sampling_rate_hz INTEGER NOT NULL CHECK (sampling_rate_hz > 0 AND sampling_rate_hz <= 10000),
    channel_labels TEXT[] NOT NULL,
    channel_data JSONB NOT NULL, -- Array of arrays: [[sample1_ch1, sample1_ch2, ...], ...]
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_eeg_device_id ON eeg_measurements(device_id);
CREATE INDEX idx_eeg_session_id ON eeg_measurements(session_id);
CREATE INDEX idx_eeg_subject_id ON eeg_measurements(subject_id);
CREATE INDEX idx_eeg_recording_start ON eeg_measurements(recording_started_at);
CREATE INDEX idx_eeg_created_at ON eeg_measurements(created_at);

-- HRV measurements table (RR intervals)
CREATE TABLE hrv_measurements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    session_id UUID REFERENCES sessions(id) ON DELETE SET NULL,
    subject_id VARCHAR(100),
    recording_started_at TIMESTAMPTZ NOT NULL,
    recording_ended_at TIMESTAMPTZ NOT NULL,
    rr_intervals_ms FLOAT[] NOT NULL, -- Array of RR intervals in milliseconds
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_hrv_device_id ON hrv_measurements(device_id);
CREATE INDEX idx_hrv_session_id ON hrv_measurements(session_id);
CREATE INDEX idx_hrv_subject_id ON hrv_measurements(subject_id);
CREATE INDEX idx_hrv_recording_start ON hrv_measurements(recording_started_at);
CREATE INDEX idx_hrv_created_at ON hrv_measurements(created_at);

-- Olfaction measurements table (smell sensor data)
CREATE TABLE olfaction_measurements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    session_id UUID REFERENCES sessions(id) ON DELETE SET NULL,
    subject_id VARCHAR(100),
    recording_started_at TIMESTAMPTZ NOT NULL,
    sensor_readings JSONB NOT NULL, -- Array of sensor values
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_olfaction_device_id ON olfaction_measurements(device_id);
CREATE INDEX idx_olfaction_session_id ON olfaction_measurements(session_id);
CREATE INDEX idx_olfaction_subject_id ON olfaction_measurements(subject_id);
CREATE INDEX idx_olfaction_recording_start ON olfaction_measurements(recording_started_at);
CREATE INDEX idx_olfaction_created_at ON olfaction_measurements(created_at);

-- Update timestamps triggers
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_devices_updated_at 
    BEFORE UPDATE ON devices 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sessions_updated_at 
    BEFORE UPDATE ON sessions 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();