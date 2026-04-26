-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Proteostasis parameters table
CREATE TABLE proteostasis_parameters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tissue_type VARCHAR(100) NOT NULL,
    d50 DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    alpha5 DOUBLE PRECISION NOT NULL DEFAULT 0.05,
    n5_critical DOUBLE PRECISION NOT NULL DEFAULT 50.0,
    beta5 DOUBLE PRECISION NOT NULL DEFAULT 0.1,
    tau5 DOUBLE PRECISION NOT NULL DEFAULT 10.0,
    gamma51 DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma52 DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma53 DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma54 DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    gamma55 DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    weight DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT valid_weights CHECK (weight >= 0.0 AND weight <= 1.0)
);

-- Create index on tissue type for faster lookups
CREATE INDEX idx_proteostasis_parameters_tissue ON proteostasis_parameters(tissue_type);

-- Proteostasis time series table
CREATE TABLE proteostasis_time_series (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subject_id VARCHAR(100) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    cell_divisions DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    chronological_time DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    d5_value DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    parameter_set_id UUID REFERENCES proteostasis_parameters(id) ON DELETE SET NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for time series queries
CREATE INDEX idx_proteostasis_ts_subject ON proteostasis_time_series(subject_id);
CREATE INDEX idx_proteostasis_ts_timestamp ON proteostasis_time_series(timestamp);
CREATE INDEX idx_proteostasis_ts_parameter ON proteostasis_time_series(parameter_set_id);
CREATE INDEX idx_proteostasis_ts_metadata ON proteostasis_time_series USING GIN (metadata);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for automatic updated_at
CREATE TRIGGER update_proteostasis_parameters_updated_at
    BEFORE UPDATE ON proteostasis_parameters
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_proteostasis_ts_updated_at
    BEFORE UPDATE ON proteostasis_time_series
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert default parameters according to PARAMETERS.md
INSERT INTO proteostasis_parameters (
    id,
    tissue_type,
    d50,
    alpha5,
    n5_critical,
    beta5,
    tau5,
    gamma51,
    gamma52,
    gamma53,
    gamma54,
    gamma55,
    weight
) VALUES 
(
    uuid_generate_v4(),
    'neuron',
    0.0,
    0.05,
    50.0,
    0.1,
    10.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.4
),
(
    uuid_generate_v4(),
    'muscle',
    0.0,
    0.05,
    50.0,
    0.1,
    10.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.2
),
(
    uuid_generate_v4(),
    'liver',
    0.0,
    0.05,
    50.0,
    0.1,
    10.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.05
),
(
    uuid_generate_v4(),
    'default',
    0.0,
    0.05,
    50.0,
    0.1,
    10.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0
);