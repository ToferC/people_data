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
    pub birth_date: NaiveDate,
    pub gender: String,

    pub travel_document_id: String,
    pub travel_document_issuer_id: Uuid, // Country

    pub travel_group_id: Uuid,

    pub approved_access_level: String, // AccessLevel
    pub approved_access_granularity: String, // Granularity
}

impl NewPerson {

    pub fn new(
        family_name: String,
        given_name: String,
        additional_names: Option<Vec<String>>,
        birth_date: NaiveDate,
        gender: String,
        travel_document_id: String,
        travel_document_issuer_id: Uuid, // Country
        travel_group_id: Uuid,
        approved_access_level: String, // AccessLevel
        approved_access_granularity: String,
    ) -> Self {
        NewPerson {
            family_name,
            given_name,
            additional_names,
            birth_date,
            gender,
            travel_document_id,
            travel_document_issuer_id,
            travel_group_id,
            approved_access_level,
            approved_access_granularity,
        }
    }
}