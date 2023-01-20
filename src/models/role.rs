/// Intermediary data structure between person and team
pub struct Role {
    pub id: Uuid,
    pub team_id: Uuid,
    // pub person_id: Uuid // --> Not sure about this
    pub title_en: String,
    pub title_fr: String,
    pub effort: f32,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

pub struct Work {
    pub id: Uuid,
    pub outcome_en: String,
    pub outcome_fr: String,
    pub start_date: NaiveDate,
    pub target_completion_data: NaiveDate,
    pub completed_date: Option<NaiveDate>,
}

pub struct WorkSkillRequirement {
    pub id: Uuid,
    pub work_id: Uuid,
    pub skill_id: Uuid,
    pub required_level: u32,
}

// Assessment of a persons work in a role
pub struct Assessment {
    pub id: Uiud,
    pub role_id: Uuid,
    pub assessor_id: Uuid,
    pub assessed_level: u32,
    pub narrative_en: Option<String>,
    pub narrative_fr: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub date_stamp: NaiveDate,
}

/// Data structure connecting persons in heirarchical relationship
pub struct ReportingRelationship {
    pub id: Uuid,
    pub reporter: Uuid,
    pub reporting_to: Uuid,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}


pub struct OrgTier {
    pub id: Uuid,
    pub organization_id: Uuid, 
    pub level: i32,
    pub name_en: String,
    pub name_fr: String,
    pub higher_tier: Option<Uuid>, // Recursive reference to OrgTier
    pub owner_id: Uuid, // References person
    pub created_at: NaiveDate,
    pub retired_at: Option<NaiveDate>,
}

pub struct Capability {
    pub id: Uuid,
    pub skill_id: Uuid,
    pub self_identified_level: u32,
}

// Enums for Capability -> shift to 0 - 4
pub enum CapabilityLevel {
    Desired,
    Novice,
    Experienced,
    Expert,
    Specialist,
}

/// Other people's validations of an individuals Capability
pub struct Validations {
    pub id: Uuid,
    pub validator_id: Uuid, // references person
    pub capability_id: Uuid,
    pub validated_level: u32,
    pub date_stamp: NaiveDate,
}

// External certifications or credentials like degrees, professional certs, etc
pub struct Credential {
    pub id: Uuid,
    pub person_id: Uuid,
    pub provider: String,
    pub description: String,
    pub received_date: NaiveDate,
    pub validated: bool,
}

pub struct Affiliation {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub role: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

// DemographicData
// EmployeeData
// DataAccess