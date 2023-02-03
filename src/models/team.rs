use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;
use rand::{Rng, thread_rng};
use crate::graphql::graphql_translate;


use crate::schema::*;
use crate::errors::CustomError;
use crate::database::connection;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "teams"]
/// Referenced by Role
pub struct Team {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub org_tier_id: Uuid,

    pub name_en: String,
    pub name_fr: String,

    pub description_en: String,
    pub description_fr: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,

    // pub milestones: Uuid // Refers to Github Milestones
}

// Non Graphql
impl Team {
    pub fn create(conn: &PgConnection, team: &NewTeam) -> FieldResult<Team> {
        let res = diesel::insert_into(teams::table)
        .values(team)
        .get_result(conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, team: &NewTeam) -> FieldResult<Team> {
        let res = teams::table
        .filter(teams::name_en.eq(&team.name_en))
        .filter(teams::name_fr.eq(&team.name_fr))
        .filter(teams::organization_id.eq(&team.organization_id))
        .distinct()
        .first(conn);
        
        let team = match res {
            Ok(p) => p,
            Err(e) => {
                // Team not found
                println!("{:?}", e);
                let p = Team::create(conn, team).expect("Unable to create team");
                p
            }
        };
        Ok(team)
    }
    
    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let res = diesel::update(teams::table)
        .filter(teams::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
/// Linked from HealthProfile
/// Linked to Trip
#[table_name = "teams"]
pub struct NewTeam {
    pub name_en: String,
    pub name_fr: String,

    pub organization_id: Uuid,
    pub org_tier_id: Uuid,
    
    pub description_en: String,
    pub description_fr: String,
}

impl NewTeam {

    pub fn new(
        name_en: String,
        name_fr: String,
        organization_id: Uuid,
        org_tier_id: Uuid,
        description_en: String,
        description_fr: String,
    ) -> Self {
        NewTeam {
            name_en,
            name_fr,
            organization_id,
            org_tier_id,
            description_en,
            description_fr,
        }
    }
}
