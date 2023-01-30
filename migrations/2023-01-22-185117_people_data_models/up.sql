-- Your SQL goes here

CREATE TABLE IF NOT EXISTS oranizations (
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
    user_id UUID UNIQUE NOT NULL,
    family_name VARCHAR NOT NULL,
    given_name VARCHAR NOT NULL,
    additional_names TEXT[],
    
    organization_id UUID NOT NULL,
    FOREIGN KEY(organization_id)
        REFERENCES organizations(id) ON DELETE RESTRICT,
    
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS roles (
    id UUID DEFAULT gen_random_uuid(),

    team_id UUID UNIQUE NOT NULL,
    FOREIGN KEY(team_id)
        REFERENCES teams(id) ON DELETE IGNORE,

    person_id UUID UNIQUE NOT NULL,
    FOREIGN KEY(person_id)
        REFERENCES persons(id) ON DELETE IGNORE,

    PRIMARY KEY (person_id, team_id),

    title_en VARCHAR(256) UNIQUE NOT NULL,
    title_fr VARCHAR(256) UNIQUE NOT NULL,
    effort FLOAT NOT NULL,

    active bool NOT NULL,
    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)

CREATE TABLE IF NOT EXISTS team_ownerships (
    id UUID DEFAULT gen_random_uuid(),

    person_id UUID NOT NULL,
    FOREIGN KEY(person_id)
        REFERENCES persons(id) ON DELETE RESTRICT,

    team_id UUID NOT NULL,
    FOREIGN KEY(team_id)
        REFERENCES teams(id) ON DELETE RESTRICT,
    
    PRIMARY KEY (person_id, team_id),
    
    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)