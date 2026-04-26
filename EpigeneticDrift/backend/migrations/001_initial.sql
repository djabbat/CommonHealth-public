-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Epigenetic Drift Counters table
CREATE TABLE epigenetic_drift_counters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    individual_id VARCHAR(255) NOT NULL,
    tissue_type VARCHAR(100) NOT NULL,
    d4_state DOUBLE PRECISION NOT NULL,
    d4_baseline DOUBLE PRECISION NOT NULL,
    beta4 DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    tau4 DOUBLE PRECISION NOT NULL DEFAULT 10.0,
    alpha4 DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    n4_star DOUBLE PRECISION NOT NULL DEFAULT 50.0,
    -- Interaction coefficients (γ) - default 0 per canonical rules
    gamma_centriolar DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma_telomere DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma_mitoros DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma_proteostasis DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma_autocatalytic DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    weight_tissue DOUBLE PRECISION NOT NULL DEFAULT 0.2,
    measured_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for efficient querying
CREATE INDEX idx_counters_individual_id ON epigenetic_drift_counters(individual_id);
CREATE INDEX idx_counters_tissue_type ON epigenetic_drift_counters(tissue_type);
CREATE INDEX idx_counters_measured_at ON epigenetic_drift_counters(measured_at);

-- Epigenetic Drift Measurements table
CREATE TABLE epigenetic_drift_measurements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    counter_id UUID NOT NULL REFERENCES epigenetic_drift_counters(id) ON DELETE CASCADE,
    measurement_type VARCHAR(50) NOT NULL CHECK (measurement_type IN ('DNAm', 'ATAC')),
    raw_data JSONB NOT NULL,
    computed_d4 DOUBLE PRECISION NOT NULL,
    measured_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for measurements
CREATE INDEX idx_measurements_counter_id ON epigenetic_drift_measurements(counter_id);
CREATE INDEX idx_measurements_measurement_type ON epigenetic_drift_measurements(measurement_type);
CREATE INDEX idx_measurements_measured_at ON epigenetic_drift_measurements(measured_at);
CREATE INDEX idx_measurements_raw_data ON epigenetic_drift_measurements USING GIN (raw_data);

-- Epigenetic Drift Parameters table
CREATE TABLE epigenetic_drift_parameters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tissue_type VARCHAR(100) NOT NULL UNIQUE,
    beta4 DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    tau4 DOUBLE PRECISION NOT NULL DEFAULT 10.0,
    alpha4 DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    n4_star DOUBLE PRECISION NOT NULL DEFAULT 50.0,
    -- Interaction coefficients (γ) - default 0 per canonical rules
    gamma_centriolar DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma_telomere DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma_mitoros DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma_proteostasis DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma_autocatalytic DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Index for parameters
CREATE INDEX idx_parameters_tissue_type ON epigenetic_drift_parameters(tissue_type);
CREATE INDEX idx_parameters_is_default ON epigenetic_drift_parameters(is_default);

-- Insert default parameters (canonical defaults from PARAMETERS.md)
INSERT INTO epigenetic_drift_parameters (
    tissue_type, beta4, tau4, alpha4, n4_star,
    gamma_centriolar, gamma_telomere, gamma_mitoros, gamma_proteostasis, gamma_autocatalytic,
    is_default
) VALUES (
    'default', 1.0, 10.0, 0.0, 50.0,
    0.0, 0.0, 0.0, 0.0, 0.0,
    TRUE
) ON CONFLICT (tissue_type) DO NOTHING;

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for automatic updated_at updates
CREATE TRIGGER update_counters_updated_at
    BEFORE UPDATE ON epigenetic_drift_counters
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_measurements_updated_at
    BEFORE UPDATE ON epigenetic_drift_measurements
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_parameters_updated_at
    BEFORE UPDATE ON epigenetic_drift_parameters
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();