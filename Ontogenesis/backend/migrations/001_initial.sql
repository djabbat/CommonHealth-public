-- Ontogenesis v4.2 initial schema
-- Developmental prequel to MCOA (0-25 years)

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Phases table - 5 neurobiological phases
CREATE TABLE phases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    age_start INTEGER NOT NULL CHECK (age_start >= 0 AND age_start <= 120),
    age_end INTEGER NOT NULL CHECK (age_end >= 0 AND age_end <= 120 AND age_end > age_start),
    neurobio_characteristics TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT valid_age_range CHECK (age_end > age_start)
);

-- Domains table - the four domains
CREATE TABLE domains (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Parameters table - quantitative parameters in each domain
CREATE TABLE parameters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    domain_id UUID NOT NULL REFERENCES domains(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    default_value DECIMAL,
    unit VARCHAR(50),
    source TEXT,
    status VARCHAR(50), -- Measured, Estimated, TBD
    min_value DECIMAL,
    max_value DECIMAL,
    gamma DECIMAL NOT NULL DEFAULT 0.0, -- γ_i coupling coefficient, default 0 per CORRECTIONS
    is_scaffold BOOLEAN NOT NULL DEFAULT FALSE, -- For scaffold counters
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(domain_id, name)
);

-- Individuals table - simulated individuals
CREATE TABLE individuals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    cohort_id UUID, -- For future cohort support
    birth_date DATE,
    sex VARCHAR(10),
    simulated BOOLEAN NOT NULL DEFAULT FALSE,
    initial_age DECIMAL(5,2) NOT NULL DEFAULT 0.0 CHECK (initial_age >= 0 AND initial_age <= 120),
    max_age DECIMAL(5,2) NOT NULL DEFAULT 120.0 CHECK (max_age >= 0 AND max_age <= 120 AND max_age >= initial_age),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Transitions table - parameter changes at specific ages
CREATE TABLE transitions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    individual_id UUID NOT NULL REFERENCES individuals(id) ON DELETE CASCADE,
    parameter_id UUID NOT NULL REFERENCES parameters(id) ON DELETE CASCADE,
    age DECIMAL(5,2) NOT NULL CHECK (age >= 0 AND age <= 120),
    value DECIMAL NOT NULL,
    change_score DECIMAL, -- LCS change score
    previous_value DECIMAL,
    domain_coupling DECIMAL, -- γ coupling from other domains
    is_significant BOOLEAN NOT NULL DEFAULT FALSE,
    fdr_q_value DECIMAL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(individual_id, parameter_id, age)
);

-- Metamorphoses table - clusters of transitions in ≥2 domains
CREATE TABLE metamorphoses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    individual_id UUID NOT NULL REFERENCES individuals(id) ON DELETE CASCADE,
    age DECIMAL(5,2) NOT NULL CHECK (age >= 0 AND age <= 120),
    cluster_radius DECIMAL NOT NULL DEFAULT 0.5, -- In years (6 months default)
    domain_count INTEGER NOT NULL CHECK (domain_count >= 2),
    is_valid BOOLEAN NOT NULL DEFAULT TRUE,
    fdr_corrected BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Junction table for metamorphosis-transition relationships
CREATE TABLE metamorphosis_transitions (
    metamorphosis_id UUID NOT NULL REFERENCES metamorphoses(id) ON DELETE CASCADE,
    transition_id UUID NOT NULL REFERENCES transitions(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY (metamorphosis_id, transition_id)
);

-- Insert initial data for domains
INSERT INTO domains (name, description) VALUES
    ('Morphology', 'Structural changes in body and organs'),
    ('Physiology', 'Functional and biochemical processes'),
    ('Psychology', 'Cognitive, emotional, and temperamental characteristics'),
    ('Sociology', 'Social roles, events, networks')
ON CONFLICT (name) DO NOTHING;

-- Insert initial data for phases (5 neurobiological phases)
INSERT INTO phases (name, age_start, age_end, neurobio_characteristics) VALUES
    ('Childhood', 0, 9, 'Maximum neuroplasticity, primary myelination, formation of basic structures'),
    ('Adolescent-Young Adult', 9, 32, 'Puberty, completion of PFC myelination, peak cognitive functions'),
    ('Adult', 32, 66, 'Relative stability, accumulation of damage, vitauct/aging balance'),
    ('Early Aging', 66, 83, 'Accelerated neurodegeneration, decreased resilience (DOSI)'),
    ('Late Aging', 83, 120, 'Plateau and divergence of homeostatic systems, DOSI critical point')
ON CONFLICT DO NOTHING;

-- Insert initial parameters from PARAMETERS.md
-- Note: domain_id will be set based on the actual domain UUIDs
-- This is a placeholder - actual insertion would require domain lookup
DO $$
DECLARE
    morph_id UUID;
    phys_id UUID;
    psych_id UUID;
    soc_id UUID;
BEGIN
    SELECT id INTO morph_id FROM domains WHERE name = 'Morphology';
    SELECT id INTO phys_id FROM domains WHERE name = 'Physiology';
    SELECT id INTO psych_id FROM domains WHERE name = 'Psychology';
    SELECT id INTO soc_id FROM domains WHERE name = 'Sociology';
    
    -- Morphology parameters
    INSERT INTO parameters (domain_id, name, unit, status) VALUES