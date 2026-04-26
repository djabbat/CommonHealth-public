-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create taxa table (CONCEPT.md §3)
CREATE TABLE taxa (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    has_hepatic_organ BOOLEAN NOT NULL,
    steroid_regulators TEXT NOT NULL,
    bbb_permeability TEXT NOT NULL,
    affect BOOLEAN NOT NULL,