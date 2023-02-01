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

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "team_ownerships"]
/// Referenced by Role
pub struct TeamOwnership {
    pub id: Uuid,
    pub person_id: Uuid,
    pub team_id: Uuid,

    pub start_datestamp: NaiveDate,
    pub end_date: Option<NaiveDate>,

    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,

    // pub milestones: Uuid // Refers to Github Milestones
}

// Non Graphql
impl TeamOwnership {
    pub fn create(conn: &PgConnection, team_ownership: &NewTeamOwnership) -> FieldResult<TeamOwnership> {
        let res = diesel::insert_into(team_ownerships::table)
        .values(team_ownership)
        .get_result(conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, team_ownership: &NewTeamOwnership) -> FieldResult<TeamOwnership> {
        let res = team_ownerships::table
        .filter(team_ownerships::name_en.eq(&team_ownership.name_en))
        .filter(team_ownerships::name_fr.eq(&team_ownership.name_fr))
        .filter(team_ownerships::organization_id.eq(&team_ownership.organization_id))
        .distinct()
        .first(conn);
        
        let team_ownership = match res {
            Ok(p) => p,
            Err(e) => {
                // TeamOwnership not found
                println!("{:?}", e);
                let p = TeamOwnership::create(conn, team_ownership).expect("Unable to create team_ownership");
                p
            }
        };
        Ok(team_ownership)
    }
    
    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let res = diesel::update(team_ownerships::table)
        .filter(team_ownerships::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
/// Linked from HealthProfile
/// Linked to Trip
#[table_name = "team_ownerships"]
pub struct NewTeamOwnership {
    pub person_id: Uuid,
    pub team_id: Uuid,

    pub start_datestamp: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

impl NewTeamOwnership {

    pub fn new(
        person_id: Uuid,
        team_id: Uuid,
        start_datestamp: NaiveDate,
        end_date: Option<NaiveDate>,
    ) -> Self {
        NewTeamOwnership {
            person_id,
            team_id,
            start_datestamp,
            end_date,
        }
    }
}
