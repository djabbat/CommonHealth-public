-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Counters table (canonical MCOA counters)
CREATE TABLE counters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    alpha DOUBLE PRECISION NOT NULL,  -- division-driven rate α_i
    beta DOUBLE PRECISION NOT NULL,   -- time-driven rate β_i
    gamma DOUBLE PRECISION NOT NULL DEFAULT 0,  -- interaction coefficient γ_i
    d_critical_default DOUBLE PRECISION NOT NULL,  -- default critical damage threshold
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Tissues table
CREATE TABLE tissues (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    l_critical DOUBLE PRECISION NOT NULL,  -- critical load threshold L_critical(tissue)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Counter-tissue specific parameters (n_i*, τ_i, w_i, D_critical)
CREATE TABLE counter_tissue_params (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    counter_id UUID NOT NULL REFERENCES counters(id) ON DELETE CASCADE,
    tissue_id UUID NOT NULL REFERENCES tissues(id) ON DELETE CASCADE,
    n_reference DOUBLE PRECISION NOT NULL,  -- n_i* for this counter in this tissue
    tau_reference DOUBLE PRECISION NOT NULL, -- τ_i for this counter in this tissue
    d_critical DOUBLE PRECISION NOT NULL,   -- tissue-specific D_critical
    weight DOUBLE PRECISION NOT NULL,       -- w_i(tissue) a-priori weight
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(counter_id, tissue_id)
);

-- Subjects table (organisms being studied)
CREATE TABLE subjects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    identifier VARCHAR(255) NOT NULL UNIQUE,
    species VARCHAR(100) NOT NULL,
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Damage measurements table
CREATE TABLE damage_measurements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subject_id UUID NOT NULL REFERENCES subjects(id) ON DELETE CASCADE,
    counter_id UUID NOT NULL REFERENCES counters(id) ON DELETE CASCADE,
    tissue_id UUID NOT NULL REFERENCES tissues(id) ON DELETE CASCADE,
    division_count DOUBLE PRECISION NOT NULL,  -- n
    time_value DOUBLE PRECISION NOT NULL,      -- t in seconds
    damage DOUBLE PRECISION NOT NULL,          -- D_i calculated
    measured_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Tissue loads table
CREATE TABLE tissue_loads (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subject_id UUID NOT NULL REFERENCES subjects(id) ON DELETE CASCADE,
    tissue_id UUID NOT NULL REFERENCES tissues(id) ON DELETE CASCADE,
    load_value DOUBLE PRECISION NOT NULL,  -- L_tissue calculated
    computed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Coupling matrix table (Γ_ij coefficients)
CREATE TABLE coupling_matrix (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    influencer_counter_id UUID NOT NULL REFERENCES counters(id) ON DELETE CASCADE,  -- j
    influenced_counter_id UUID NOT NULL REFERENCES counters(id) ON DELETE CASCADE,  -- i
    gamma_ij DOUBLE PRECISION NOT NULL,  -- Γ_ij coupling coefficient
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(influencer_counter_id, influenced_counter_id)
);

-- Create indexes for performance
CREATE INDEX idx_damage_measurements_subject ON damage_measurements(subject_id);
CREATE INDEX idx_damage_measurements_counter ON damage_measurements(counter_id);
CREATE INDEX idx_damage_measurements_tissue ON damage_measurements(tissue_id);
CREATE INDEX idx_damage_measurements_measured_at ON damage_measurements(measured_at);

CREATE INDEX idx_tissue_loads_subject ON tissue_loads(subject_id);
CREATE INDEX idx_tissue_loads_tissue ON tissue_loads(tissue_id);
CREATE INDEX idx_tissue_loads_computed_at ON tissue_loads(computed_at);

CREATE INDEX idx_counter_tissue_params_counter ON counter_tissue_params(counter_id);
CREATE INDEX idx_counter_tissue_params_tissue ON counter_tissue_params(tissue_id);

CREATE INDEX idx_coupling_matrix_influencer ON coupling_matrix(influencer_counter_id);
CREATE INDEX idx_coupling_matrix_influenced ON coupling_matrix(influenced_counter_id);

-- Insert initial canonical counters (from PARAMETERS.md)
INSERT INTO counters (name, description, alpha, beta, gamma, d_critical_default) VALUES
    ('telomere', 'Telomere shortening counter', 0.02, 0.002, 0, 0.85),
    ('centriolar_polyglutamylation', 'Centriolar polyglutamylation counter', 0.015, 0.005, 0, 0.80),
    ('mitochondrial_ros', 'Mitochondrial ROS / mtDNA damage counter', 0, 0.01, 0, 0.75),
    ('epigenetic_drift', 'Epigenetic drift counter', 0, 0.008, 0, 0.70),
    ('proteostasis_collapse', 'Proteostasis collapse counter', 0.005, 0.006, 0, 0.75);

-- Insert initial tissues (from PARAMETERS.md)
INSERT INTO tissues (name, description, l_critical) VALUES
    ('fibroblast_dermal', 'Dermal fibroblast', 0.60),
    ('hsc', 'Hematopoietic stem cell', 0.60),
    ('neuron_postmitotic', 'Post-mitotic neuron', 0.60),
    ('hepatocyte', 'Liver hepatocyte', 0.60),
    ('pancreatic_beta_cell', 'Pancreatic β-cell', 0.60),
    ('cd8_t_memory', 'CD8+ T-memory cell', 0.60);

-- Insert counter-tissue parameters (simplified initial values)
-- Note: These are placeholder values; real values should be loaded from PARAMETERS.md
INSERT INTO counter_tissue_params (counter_id, tissue_id, n_reference, tau_reference, d_critical, weight)
SELECT 
    c.id,
    t.id,
    CASE 
        WHEN c.name = 'telomere' AND t.name = 'fibroblast_dermal' THEN 50
        WHEN c.name = 'centriolar_polyglutamylation' AND t.name = 'hsc' THEN 65
        ELSE 40
    END,
    CASE 
        WHEN c.name IN ('telomere', 'epigenetic_drift') THEN 31536000  -- 1 year in seconds
        WHEN c.name = 'centriolar_polyglutamylation' THEN 15811200    -- 6 months in seconds
        WHEN c.name = 'mitochondrial_ros' AND t.name = 'hepatocyte' THEN 1209600  -- 14 days
        ELSE 2592000  -- 30 days
    END,
    c.d_critical_default,
    CASE 
        WHEN c.name = 'telomere' AND t.name = 'fibroblast_dermal' THEN 0.40
        WHEN c.name = 'centriolar_polyglutamylation' AND t.name = 'hsc' THEN 0.40
        WHEN c.name = 'mitochondrial_ros' AND t.name = 'neuron_postmitotic' THEN 0.35
        WHEN c.name = 'epigenetic_drift' AND t.name = 'pancreatic_beta_cell' THEN 0.40
        WHEN c.name = 'proteostasis_collapse' AND t.name = 'hepatocyte' THEN 0.30
        ELSE 0.15
    END
FROM counters c
CROSS JOIN tissues t
WHERE (c.name, t.name) IN (
    ('telomere', 'fibroblast_dermal'),
    ('telomere', 'hsc'),
    ('telomere', 'neuron_postmitotic'),
    ('telomere', 'hepatocyte'),
    ('telomere', 'pancreatic_beta_cell'),
    ('telomere', 'cd8_t_memory'),
    ('centriolar_polyglutamylation', 'fibroblast_dermal'),
    ('centriolar_polyglutamylation', 'hsc'),
    ('centriolar_polyglutamylation', 'neuron_postmitotic'),
    ('centriolar_polyglutamylation', 'hepatocyte'),
    ('centriolar_polyglutamylation', 'pancreatic_beta_cell'),
    ('centriolar_polyglutamylation', 'cd8_t_memory'),
    ('mitochondrial_ros', 'fibroblast_dermal'),
    ('mitochondrial_ros', 'hsc'),
    ('mitochondrial_ros', 'neuron_postmitotic'),
    ('mitochondrial_ros', 'hepatocyte'),
    ('mitochondrial_ros', 'pancreatic_beta_cell'),
    ('mitochondrial_ros', 'cd8_t_memory'),
    ('epigenetic_drift', 'fibroblast_dermal'),
    ('epigenetic_drift', 'hsc'),
    ('epigenetic_drift', 'neuron_postmitotic'),
    ('epigenetic_drift', 'hepatocyte'),
    ('epigenetic_drift', 'pancreatic_beta_cell'),
    ('epigenetic_drift', 'cd8_t_memory'),
    ('proteostasis_collapse', 'fibroblast_dermal'),
    ('proteostasis_collapse', 'hsc'),
    ('proteostasis_collapse', 'neuron_postmitotic'),
    ('proteostasis_collapse', 'hepatocyte'),
    ('proteostasis_collapse', 'pancreatic_beta_cell'),
    ('proteostasis_collapse', 'cd8_t_memory')
);

-- Insert initial coupling matrix (Γ) from PARAMETERS.md
INSERT INTO coupling_matrix (influencer_counter_id, influenced_counter_id, gamma_ij)
SELECT 
    influencer.id,
    influenced.id,
    CASE 
        WHEN influencer.name = 'mitochondrial_ros' AND influenced.name = 'telomere' THEN 0.30
        WHEN influencer.name = 'mitochondrial_ros' AND influenced.name = 'epigenetic_drift' THEN 0.30
        WHEN influencer.name = 'epigenetic_drift' AND influenced.name = 'centriolar_polyglutamylation' THEN 0.20
        WHEN influencer.name = 'mitochondrial_ros' AND influenced.name = 'centriolar_polyglutamylation' THEN 0.10
        WHEN influencer.name = 'mitochondrial_ros' AND influenced.name = 'proteostasis_collapse' THEN 0.20
        WHEN influencer.name = 'proteostasis_collapse' AND influenced.name = 'centriolar_polyglutamylation' THEN 0.05
        WHEN influencer.name = 'centriolar_polyglutamylation' AND influenced.name = 'epigenetic_drift' THEN 0.05
        WHEN influencer.name = 'centriolar_polyglutamylation' AND influenced.name = 'proteostasis_collapse' THEN 0.10
        WHEN influencer.name = 'telomere' AND influenced.name = 'epigenetic_drift' THEN 0.05
        ELSE 0.00
    END
FROM counters influencer
CROSS JOIN counters influenced
WHERE influencer.id != influenced.id;