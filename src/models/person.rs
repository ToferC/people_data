use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;
use rand::{Rng, thread_rng};

use crate::graphql::graphql_translate;
use crate::errors::CustomError;

use crate::database::connection;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "persons"]
/// Referenced by Team
/// Referenced by ReportingRelationship
pub struct Person {
    pub id: Uuid,
    pub user_id: Uuid,
    pub family_name: String,
    pub given_name: String,

    pub organization_id: Uuid, // Organization 

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}


// Non Graphql
impl Person {
    pub fn create(person: &NewPerson) -> FieldResult<Person> {
        let conn = connection()?;
        let res = diesel::insert_into(persons::table)
        .values(person)
        .get_result(&conn);
        
        graphql_translate(res)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = connection()?;
        let persons = persons::table.load::<Person>(&conn)?;
        Ok(persons)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = connection()?;
        let person = persons::table.filter(persons::id.eq(id)).first(&conn)?;
        Ok(person)
    }
    
    pub fn get_or_create(person: &NewPerson) -> FieldResult<Person> {
        let conn = connection()?;
        let res = persons::table
        .filter(persons::family_name.eq(&person.family_name))
        .distinct()
        .first(&conn);
        
        let person = match res {
            Ok(p) => p,
            Err(e) => {
                // Person not found
                println!("{:?}", e);
                let p = Person::create(person).expect("Unable to create person");
                p
            }
        };
        Ok(person)
    }
    
    pub fn update(&self) -> FieldResult<Self> {
        let conn = connection()?;

        let res = diesel::update(persons::table)
        .filter(persons::id.eq(&self.id))
        .set(self)
        .get_result(&conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
/// Referenced by Roles, TeamOwnership, OrgOwnership
#[table_name = "persons"]
pub struct NewPerson {
    pub user_id: Uuid,
    pub family_name: String,
    pub given_name: String,
    pub organization_id: Uuid, // Organization
}

impl NewPerson {

    pub fn new(
        user_id: Uuid,
        family_name: String,
        given_name: String,
        organization_id: Uuid, // Organizatio
    ) -> Self {
        NewPerson {
            user_id,
            family_name,
            given_name,
            organization_id,
        }
    }
}
