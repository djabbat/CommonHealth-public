-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tissues table
CREATE TABLE tissues (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    mitotic_index DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    metabolic_rate DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    weight_w3 DOUBLE PRECISION,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_tissues_name ON tissues(name);
CREATE INDEX idx_tissues_mitotic_index ON tissues(mitotic_index);

-- Counter3Parameters table
CREATE TABLE counter3_parameters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tissue_id UUID NOT NULL REFERENCES tissues(id) ON DELETE CASCADE,
    d3_0 DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    alpha3 DOUBLE PRECISION NOT NULL DEFAULT 0.001,
    n3_star DOUBLE PRECISION NOT NULL DEFAULT 1000.0,
    beta3 DOUBLE PRECISION NOT NULL DEFAULT 0.01,
    tau3 DOUBLE PRECISION NOT NULL DEFAULT 30.0,
    gamma3 DOUBLE PRECISION NOT NULL DEFAULT 0.0,  -- Default 0 per CORRECTIONS_2026-04-22
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tissue_id, created_at)
);

CREATE INDEX idx_counter3_parameters_tissue_id ON counter3_parameters(tissue_id);
CREATE INDEX idx_counter3_parameters_created_at ON counter3_parameters(created_at);

-- Counter3Records table
CREATE TABLE counter3_records (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tissue_id UUID NOT NULL REFERENCES tissues(id) ON DELETE CASCADE,
    n_cell_divisions DOUBLE PRECISION NOT NULL,
    t_time DOUBLE PRECISION NOT NULL,  -- Time in years
    d3_value DOUBLE PRECISION NOT NULL,
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_counter3_records_tissue_id ON counter3_records(tissue_id);
CREATE INDEX idx_counter3_records_created_at ON counter3_records(created_at);
CREATE INDEX idx_counter3_records_n_t ON counter3_records(n_cell_divisions, t_time);

-- Update triggers for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_tissues_updated_at BEFORE UPDATE ON tissues
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_counter3_parameters_updated_at BEFORE UPDATE ON counter3_parameters
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_counter3_records_updated_at BEFORE UPDATE ON counter3_records
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert some initial tissues
INSERT INTO tissues (name, description, mitotic_index, metabolic_rate, weight_w3) VALUES
    ('Liver', 'Hepatic tissue', 0.02, 1.0, 0.15),
    ('Brain', 'Neuronal tissue', 0.001, 0.2, 0.25),
    ('Muscle', 'Skeletal muscle', 0.01, 0.5, 0.10),
    ('Intestine', 'Intestinal epithelium', 0.1, 1.5, 0.05),
    ('Skin', 'Dermal tissue', 0.05, 0.3, 0.08)
ON CONFLICT (name) DO NOTHING;

-- Insert default parameters for each tissue
INSERT INTO counter3_parameters (tissue_id, d3_0, alpha3, n3_star, beta3, tau3, gamma3)
SELECT 
    t.id,
    0.001,  -- d3_0
    CASE WHEN t.mitotic_index > 0.05 THEN 0.002 ELSE 0.001 END,  -- alpha3
    1000.0, -- n3_star
    0.01,   -- beta3
    CASE WHEN t.name = 'Brain' THEN 50.0 ELSE 30.0 END,  -- tau3 longer for post-mitotic
    0.0     -- gamma3 = 0 per canonical rules
FROM tissues t
ON CONFLICT (tissue_id, created_at) DO NOTHING;