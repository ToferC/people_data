use std::fmt::Debug;

use chrono::{prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use async_graphql::*;
use rand::{Rng, thread_rng};

use crate::graphql::graphql_translate;

use crate::database::connection;
use crate::errors::CustomError;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[table_name = "org_tiers"]
pub struct OrgTier {
    pub id: Uuid,
    pub organization_id: Uuid, // Organization
    pub tier_level: i32,
    pub name_en: String,
    pub name_fr: String,
    pub parent_tier: Option<Uuid>, // Recursive reference to OrgTier
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub retired_at: Option<NaiveDateTime>,
}

// Non Graphql
impl OrgTier {
    pub fn create(conn: &PgConnection, org_tier: &NewOrgTier) -> FieldResult<OrgTier> {
        let res = diesel::insert_into(org_tiers::table)
        .values(org_tier)
        .get_result(conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, org_tier: &NewOrgTier) -> FieldResult<OrgTier> {
        let res = org_tiers::table
        .filter(org_tiers::name_en.eq(&org_tier.name_en))
        .distinct()
        .first(conn);
        
        let org_tier = match res {
            Ok(p) => p,
            Err(e) => {
                // OrgTier not found
                println!("{:?}", e);
                let p = OrgTier::create(conn, org_tier).expect("Unable to create org_tier");
                p
            }
        };
        Ok(org_tier)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = connection()?;
        let org_tiers = org_tiers::table.load::<OrgTier>(&conn)?;
        Ok(org_tiers)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = connection()?;
        let org_tier = org_tiers::table.filter(org_tiers::id.eq(id)).first(&conn)?;
        Ok(org_tier)
    }
    
    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let res = diesel::update(org_tiers::table)
        .filter(org_tiers::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
#[table_name = "org_tiers"]
pub struct NewOrgTier {
    pub organization_id: Uuid, // Organization
    pub tier_level: i32,
    pub name_en: String,
    pub name_fr: String,
    pub parent_tier: Option<Uuid>, // Recursive reference to OrgTier
}

impl NewOrgTier {

    pub fn new(
        organization_id: Uuid, // Organization
        tier_level: i32,
        name_en: String,
        name_fr: String,
        parent_tier: Option<Uuid>, // Recursive reference to OrgTier
    ) -> Self {
        NewOrgTier {
            organization_id,
            tier_level,
            name_en,
            name_fr,
            parent_tier,
        }
    }
}
