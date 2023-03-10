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
#[table_name = "roles"]
/// Intermediary data structure between Person and team
/// Referenced by Person
pub struct Role {
    pub id: Uuid,
    pub person_id: Uuid,
    pub team_id: Uuid,
    pub title_en: String,
    pub title_fr: String,
    pub effort: f64,
    pub active: bool,
    pub start_datestamp: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


// Non Graphql
impl Role {
    pub fn create(conn: &PgConnection, role: &NewRole) -> FieldResult<Role> {
        let res = diesel::insert_into(roles::table)
        .values(role)
        .get_result(conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, role: &NewRole) -> FieldResult<Role> {
        let res = roles::table
        .filter(roles::person_id.eq(&role.person_id))
        .distinct()
        .first(conn);
        
        let role = match res {
            Ok(p) => p,
            Err(e) => {
                // Role not found
                println!("{:?}", e);
                let p = Role::create(conn, role).expect("Unable to create role");
                p
            }
        };
        Ok(role)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = connection()?;
        let roles = roles::table.load::<Role>(&conn)?;
        Ok(roles)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = connection()?;
        let role = roles::table.filter(roles::id.eq(id)).first(&conn)?;
        Ok(role)
    }
    
    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let res = diesel::update(roles::table)
        .filter(roles::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
#[table_name = "roles"]
pub struct NewRole {
    pub id: Uuid,
    pub person_id: Uuid,
    pub team_id: Uuid,
    pub title_en: String,
    pub title_fr: String,
    pub effort: f64,
    pub active: bool,
    pub start_datestamp: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,
}

impl NewRole {

    pub fn new(
        id: Uuid,
        person_id: Uuid,
        team_id: Uuid,
        title_en: String,
        title_fr: String,
        effort: f64,
        active: bool,
        start_datestamp: NaiveDateTime,
        end_date: Option<NaiveDateTime>,
    ) -> Self {
        NewRole {
            id,
            person_id,
            team_id,
            title_en,
            title_fr,
            effort,
            active,
            start_datestamp,
            end_date,
        }
    }
}
