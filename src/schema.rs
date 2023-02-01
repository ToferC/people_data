table! {
    email_verification_code (id) {
        id -> Uuid,
        email_address -> Varchar,
        activation_code -> Varchar,
        expires_on -> Timestamp,
    }
}

table! {
    org_tier_ownerships (id) {
        id -> Uuid,
        owner_id -> Uuid,
        org_tier_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

table! {
    org_tiers (id) {
        id -> Uuid,
        organization_id -> Uuid,
        tier_level -> Int4,
        name_en -> Varchar,
        name_fr -> Varchar,
        parent_tier -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

table! {
    organizations (id) {
        id -> Uuid,
        name_en -> Varchar,
        name_fr -> Varchar,
        acronym_en -> Varchar,
        acronym_fr -> Varchar,
        otg_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

table! {
    password_reset_token (id) {
        id -> Uuid,
        email_address -> Varchar,
        reset_token -> Varchar,
        expires_on -> Timestamp,
    }
}

table! {
    persons (id) {
        id -> Uuid,
        user_id -> Uuid,
        family_name -> Varchar,
        given_name -> Varchar,
        additional_names -> Nullable<Array<Text>>,
        organization_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

table! {
    roles (id) {
        id -> Uuid,
        person_id -> Uuid,
        team_id -> Uuid,
        title_en -> Varchar,
        title_fr -> Varchar,
        effort -> Float8,
        active -> Bool,
        start_datestamp -> Timestamp,
        end_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    team_ownerships (id) {
        id -> Uuid,
        person_id -> Uuid,
        team_id -> Uuid,
        start_datestamp -> Timestamp,
        end_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    teams (id) {
        id -> Uuid,
        organization_id -> Uuid,
        org_tier_id -> Uuid,
        name_en -> Varchar,
        name_fr -> Varchar,
        description_en -> Text,
        description_fr -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        retired_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        hash -> Bytea,
        salt -> Varchar,
        email -> Varchar,
        user_name -> Varchar,
        slug -> Varchar,
        created_at -> Timestamp,
        role -> Varchar,
        validated -> Bool,
    }
}

joinable!(org_tier_ownerships -> org_tiers (org_tier_id));
joinable!(org_tier_ownerships -> persons (owner_id));
joinable!(org_tiers -> organizations (organization_id));
joinable!(persons -> organizations (organization_id));
joinable!(roles -> persons (person_id));
joinable!(roles -> teams (team_id));
joinable!(team_ownerships -> persons (person_id));
joinable!(team_ownerships -> teams (team_id));
joinable!(teams -> org_tiers (org_tier_id));
joinable!(teams -> organizations (organization_id));

allow_tables_to_appear_in_same_query!(
    email_verification_code,
    org_tier_ownerships,
    org_tiers,
    organizations,
    password_reset_token,
    persons,
    roles,
    team_ownerships,
    teams,
    users,
);
