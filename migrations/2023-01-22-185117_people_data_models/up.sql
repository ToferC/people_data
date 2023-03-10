-- Your SQL goes here

CREATE TABLE IF NOT EXISTS organizations (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name_en VARCHAR(256) UNIQUE NOT NULL,
    name_fr VARCHAR(256) UNIQUE NOT NULL,
    acronym_en VARCHAR(16) UNIQUE NOT NULL,
    acronym_fr VARCHAR(16) UNIQUE NOT NULL,
    org_type VARCHAR(32) NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS persons (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id UUID UNIQUE NOT NULL,
    family_name VARCHAR NOT NULL,
    given_name VARCHAR NOT NULL,
    
    organization_id UUID NOT NULL,
    FOREIGN KEY(organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,
    
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS org_tiers (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    organization_id UUID NOT NULL,
    FOREIGN KEY(organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,
    
    tier_level INT NOT NULL,
    name_en VARCHAR(256) UNIQUE NOT NULL,
    name_fr VARCHAR(256) UNIQUE NOT NULL,
    parent_tier UUID,
    FOREIGN KEY(parent_tier)
        REFERENCES org_tiers(id) ON DELETE RESTRICT,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS org_tier_ownerships (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    owner_id UUID NOT NULL,
    FOREIGN KEY(owner_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    org_tier_id UUID NOT NULL,
    FOREIGN KEY(org_tier_id)
        REFERENCES org_tiers(id) ON DELETE RESTRICT,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS teams (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    organization_id UUID NOT NULL,
    FOREIGN KEY(organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,

    org_tier_id UUID NOT NULL,
    FOREIGN KEY(org_tier_id)
        REFERENCES org_tiers(id) ON DELETE RESTRICT,

    name_en VARCHAR(256) UNIQUE NOT NULL,
    name_fr VARCHAR(256) UNIQUE NOT NULL,
    description_en TEXT NOT NULL,
    description_fr TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    retired_at TIMESTAMP DEFAULT NULL

);

CREATE TABLE IF NOT EXISTS roles (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    person_id UUID UNIQUE NOT NULL,
    FOREIGN KEY(person_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    team_id UUID UNIQUE NOT NULL,
    FOREIGN KEY(team_id)
        REFERENCES teams(id) ON DELETE RESTRICT,

    title_en VARCHAR(256) UNIQUE NOT NULL,
    title_fr VARCHAR(256) UNIQUE NOT NULL,
    effort FLOAT NOT NULL,
    active bool NOT NULL,
    start_datestamp TIMESTAMP NOT NULL,
    end_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS team_ownerships (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,

    person_id UUID NOT NULL,
    FOREIGN KEY(person_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    team_id UUID NOT NULL,
    FOREIGN KEY(team_id)
        REFERENCES teams(id) ON DELETE RESTRICT,
        
    start_datestamp TIMESTAMP NOT NULL,
    end_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)