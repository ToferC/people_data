-- Your SQL goes here

CREATE TABLE IF NOT EXISTS oranizations (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name_en VARCHAR(256) UNIQUE NOT NULL,
    name_fr VARCHAR(256) UNIQUE NOT NULL,
    acronym_en VARCHAR(16) UNIQUE NOT NULL,
    acronym_fr VARCHAR(16) UNIQUE NOT NULL
)

CREATE TABLE IF NOT EXISTS persons (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name_en VARCHAR(256) UNIQUE NOT NULL,
    name_fr VARCHAR(256) UNIQUE NOT NULL,
    acronym_en VARCHAR(16) UNIQUE NOT NULL,
    acronym_fr VARCHAR(16) UNIQUE NOT NULL
)

CREATE TABLE IF NOT EXISTS teams (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name_en VARCHAR(256) UNIQUE NOT NULL,
    name_fr VARCHAR(256) UNIQUE NOT NULL,
    organization_id UUID UNIQUE NOT NULL,
    description_en TEXT NOT NULL,
    description_fr TEXT NOT NULL,
)

CREATE TABLE IF NOT EXISTS persons (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    family_name VARCHAR NOT NULL,
    given_name VARCHAR NOT NULL,
    additional_names TEXT[],
    birth_date DATE NOT NULL,
    gender VARCHAR NOT NULL,
    
    travel_document_id VARCHAR NOT NULL,
    travel_document_issuer_id UUID NOT NULL,
    travel_group_id UUID NOT NULL,
    approved_access_level VARCHAR NOT NULL,
    approved_access_granularity VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);