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
#[table_name = "roles"]
/// Intermediary data structure between Person and team
/// Referenced by Person
pub struct Role {
    pub id: Uuid,
    pub team_id: Uuid,
    pub person_id: Uuid,
    pub title_en: String,
    pub title_fr: String,
    pub effort: f32,
    pub active: bool,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
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
        .filter(roles::family_name.eq(&role.family_name))
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
        let conn = database::connection()?;
        let roles = roles::table.load::<Role>(&conn)?;
        Ok(roles)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = database::connection()?;
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
    pub team_id: Uuid,
    pub title_en: String,
    pub title_fr: String,
    pub effort: f32,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

impl NewRole {

    pub fn new(
        team_id: Uuid,
        title_en: String,
        title_fr: String,
        effort: f32,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        created_at: NaiveDate,
        updated_at: NaiveDate,
    ) -> Self {
        NewRole {
            team_id,
            title_en,
            title_fr,
            effort,
            start_date,
            end_date,
            created_at,
            updated_at,
        }
    }
}
