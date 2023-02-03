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
use crate::database::connection;
use crate::errors::CustomError;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "team_ownerships"]
/// Referenced by Role
pub struct TeamOwnership {
    pub id: Uuid,
    pub person_id: Uuid,
    pub team_id: Uuid,

    pub start_datestamp: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

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
        .filter(team_ownerships::person_id.eq(&team_ownership.person_id))
        .filter(team_ownerships::team_id.eq(&team_ownership.team_id))
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

    pub start_datestamp: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,
}

impl NewTeamOwnership {

    pub fn new(
        person_id: Uuid,
        team_id: Uuid,
        start_datestamp: NaiveDateTime,
        end_date: Option<NaiveDateTime>,
    ) -> Self {
        NewTeamOwnership {
            person_id,
            team_id,
            start_datestamp,
            end_date,
        }
    }
}
