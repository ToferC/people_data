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
#[table_name = "org_tiers"]
pub struct OrgTier {
    pub id: Uuid,
    pub organization_id: Uuid, // Organization
    pub level: i32,
    pub name_en: String,
    pub name_fr: String,
    pub parent_tier: Option<Uuid>, // Recursive reference to OrgTier
    pub owner_id: Uuid, // References person
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub retired_at: Option<NaiveDate>,
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
        .filter(org_tiers::family_name.eq(&org_tier.family_name))
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
        let conn = database::connection()?;
        let org_tiers = org_tiers::table.load::<OrgTier>(&conn)?;
        Ok(org_tiers)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = database::connection()?;
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
    pub level: i32,
    pub name_en: String,
    pub name_fr: String,
    pub parent_tier: Option<Uuid>, // Recursive reference to OrgTier
    pub owner_id: Uuid, // References person
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub retired_at: Option<NaiveDate>,
}

impl NewOrgTier {

    pub fn new(
        organization_id: Uuid, // Organization
        level: i32,
        name_en: String,
        name_fr: String,
        parent_tier: Option<Uuid>, // Recursive reference to OrgTier
        owner_id: Uuid, // References person
        created_at: NaiveDate,
        updated_at: NaiveDate,
        retired_at: Option<NaiveDate>,
    ) -> Self {
        NewOrgTier {
            organization_id,
            level,
            name_en,
            name_fr,
            parent_tier,
            owner_id,
            created_at,
            updated_at,
            retired_at,
        }
    }
}
