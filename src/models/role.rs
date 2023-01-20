/// Intermediary data structure between person and team
/// Referenced by Person
pub struct Role {
    pub id: Uuid,
    pub team_id: Uuid,
    pub title_en: String,
    pub title_fr: String,
    pub effort: f32,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

pub struct EmployeeInformation {
    pub id: Uuid,
    pub person_id: Uuid,
    pub title_en: String,
    pub title_fr: String,
    pub group: String,
    pub level: u32,
    pub hr_state: String,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    // salary from a separate API call
}

pub struct ContactInformation {
    pub id: Uuid,
    pub person_id: Uuid,
    pub email: String,
    pub phone: String,
    pub work_address: String,
}

pub struct DataAccess {
    pub id: Uuid,
    pub person_id: Uuid,
    pub approved_access_level: String, // AccessLevel
    pub approved_access_granularity: String, // Granularity
}

pub struct DemographicData {
    pub birth_date: NaiveDate,
    pub gender: String,
    pub sexuality: String,
    pub disability: bool,
    pub ethnicity: String,
}

pub struct Work {
    pub id: Uuid,
    pub person_id: Option<Uuid>, // Person
    pub role_id: Option<Uuid>, // Role
    pub outcome_en: String,
    pub outcome_fr: String,
    pub start_date: NaiveDate,
    pub target_completion_data: NaiveDate,
    pub work_status: usize,
    pub completed_date: Option<NaiveDate>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

pub enum WorkStatus {
    Planning, // 0
    InProgress, // 1
    Complete, // 2
    Blocked, // 3
}

pub struct WorkSkillRequirement {
    pub id: Uuid,
    pub work_id: Uuid, // Work
    pub skill_id: Uuid, // Skill
    pub required_level: u32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
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
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,

}

/// Data structure connecting persons in heirarchical relationship
pub struct ReportingRelationship {
    pub id: Uuid,
    pub reporter: Uuid, // Person
    pub reporting_to: Uuid, // Person
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}


pub struct OrgTier {
    pub id: Uuid,
    pub organization_id: Uuid, // Organization
    pub level: i32,
    pub name_en: String,
    pub name_fr: String,
    pub higher_tier: Option<Uuid>, // Recursive reference to OrgTier
    pub owner_id: Uuid, // References person
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub retired_at: Option<NaiveDate>,
}

pub struct Capability {
    pub id: Uuid,
    pub person_id: Uuid, // Person
    pub skill_id: Uuid, // Skill
    pub self_identified_level: u32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
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
    pub validator_id: Uuid, // Person
    pub capability_id: Uuid, // Capability
    pub validated_level: u32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

// External certifications or credentials like degrees, professional certs, etc
pub struct Credential {
    pub id: Uuid,
    pub person_id: Uuid,
    pub provider: String,
    pub description: String,
    pub received_date: NaiveDate,
    pub validated: bool,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

pub struct Affiliation {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub role: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

// DemographicData
// EmployeeData
// DataAccess