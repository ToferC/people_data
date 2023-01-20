use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
//use juniper::{FieldResult};
use uuid::Uuid;

use async_graphql::*;

use crate::graphql::graphql_translate;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[table_name = "organizations"]
/// Represents an insertable Organization
pub struct NewOrganization {
    pub name_en: String,
    pub name_fr: String,
    pub acronym_en: String,
    pub acronym_fr: String,
    pub org_type: String,
}

impl NewOrganization {
    pub fn new(
        name_en: String,
        name_fr: String,
        acronym_en: String,
        acronym_fr: String,
        org_type: String,

    ) -> Self {
        NewOrganization {
            name_en,
            name_fr,
            acronym_en,
            acronym_fr,
            org_type,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, SimpleObject)]
#[table_name = "organizations"]
/// Should get this from an API or have standard data
/// Now pre-loaded as prt of context
pub struct Organization {
    pub id: Uuid,
    pub name_en: String,
    pub name_fr: String,
    pub acroynm_en: String,
    pub acronym_fr: String,
    pub org_type: String,
}

impl Organization {
    pub fn create(conn: &PgConnection, organization: &NewOrganization) -> FieldResult<Organization> {
        let res = diesel::insert_into(organizations::table)
            .values(organization)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn get_by_id(conn: &PgConnection, id: &Uuid) -> FieldResult<Organization> {
        let res = organizations::table.filter(organizations::id.eq(id))
            .first(conn);

        graphql_translate(res)
    }

    pub fn load_into_hash(conn: &PgConnection) -> HashMap<Uuid, Organization> {
        let res = organizations::table
            .load::<Organization>(conn)
            .expect("Unable to load organizations");

        let mut organizations: HashMap<Uuid, Organization> = HashMap::new();
        for c in res {
            organizations.insert(c.id, c);
        };

        organizations 
    }
}