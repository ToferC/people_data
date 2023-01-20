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
#[table_name = "persons"]
/// Referenced by Team
/// Referenced by ReportingRelationship
pub struct Person {
    pub id: Uuid,
    pub family_name: String,
    pub given_name: String,
    pub additional_names: Option<Vec<String>>,

    pub organization_id: Uuid, // Organization
    
    pub responsible_for_teams: Vec<Uuid>, // Vec<Team>
    pub role_ids: Vec<Uuid>, // Vec<Role>    

    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}


// Non Graphql
impl Person {
    pub fn create(conn: &PgConnection, person: &NewPerson) -> FieldResult<Person> {
        let res = diesel::insert_into(persons::table)
        .values(person)
        .get_result(conn);
        
        graphql_translate(res)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = database::connection()?;
        let persons = persons::table.load::<Person>(&conn)?;
        Ok(persons)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let person = persons::table.filter(persons::id.eq(id)).first(&conn)?;
        Ok(person)
    }
    
    pub fn get_or_create(conn: &PgConnection, person: &NewPerson) -> FieldResult<Person> {
        let res = persons::table
        .filter(persons::family_name.eq(&person.family_name))
        .distinct()
        .first(conn);
        
        let person = match res {
            Ok(p) => p,
            Err(e) => {
                // Person not found
                println!("{:?}", e);
                let p = Person::create(conn, person).expect("Unable to create person");
                p
            }
        };
        Ok(person)
    }
    
    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let res = diesel::update(persons::table)
        .filter(persons::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
/// Linked from HealthProfile
/// Linked to Trip
#[table_name = "persons"]
pub struct NewPerson {
    pub family_name: String,
    pub given_name: String,
    pub additional_names: Option<Vec<String>>,

    pub organization_id: Uuid, // Organization
    
    pub responsible_for_teams: Vec<Uuid>, // Vec<Team>
    pub role_ids: Vec<Uuid>, // Vec<Role>    

    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

impl NewPerson {

    pub fn new(
        family_name: String,
        given_name: String,
        additional_names: Option<Vec<String>>,
        organization_id: Uuid, // Organizatio
        responsible_for_teams: Vec<Uuid>, // Vec<Team>
        role_ids: Vec<Uuid>, // Vec<Role> 
        created_at: NaiveDate,
        updated_at: NaiveDate,
    ) -> Self {
        NewPerson {
            family_name,
            given_name,
            additional_names,
            organization_id,
            responsible_for_teams,
            role_ids,
            created_at,
            updated_at,
        }
    }
}
