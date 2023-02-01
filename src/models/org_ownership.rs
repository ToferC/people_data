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
#[table_name = "org_tier_ownerships"]
pub struct OrgOwnership {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub org_tier_id: Uuid,

    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub retired_at: Option<Natier_iveDate>,
}

// Non Graphql
impl OrgOwnership {
    pub fn create(conn: &PgConnection, org_tier_ownership: &NewOrgOwnership) -> FieldResult<OrgOwnership> {
        let res = diesel::insert_into(org_tier_ownerships::table)
        .values(org_tier_ownership)
        .get_result(conn);
        
        graphql_translate(res)
    }
    
    pub fn get_or_create(conn: &PgConnection, org_tier_ownership: &NewOrgOwnership) -> FieldResult<OrgOwnership> {
        let res = org_tier_ownerships::table
        .filter(org_tier_ownerships::family_name.eq(&org_tier_ownership.family_name))
        .distinct()
        .first(conn);
        
        let org_tier_ownership = match res {
            Ok(p) => p,
            Err(e) => {
                // OrgOwnership not found
                println!("{:?}", e);
                let p = OrgOwnership::create(conn, org_tier_ownership).expect("Unable to create org_tier_ownership");
                p
            }
        };
        Ok(org_tier_ownership)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = database::connection()?;
        let org_tier_ownerships = org_tier_ownerships::table.load::<OrgOwnership>(&conn)?;
        Ok(org_tier_ownerships)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let org_tier_ownership = org_tier_ownerships::table.filter(org_tier_ownerships::id.eq(id)).first(&conn)?;
        Ok(org_tier_ownership)
    }
    
    pub fn update(&self, conn: &PgConnection) -> FieldResult<Self> {
        let res = diesel::update(org_tier_ownerships::table)
        .filter(org_tier_ownerships::id.eq(&self.id))
        .set(self)
        .get_result(conn)?;
        
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, SimpleObject)]
#[table_name = "org_tier_ownerships"]
pub struct NewOrgOwnership {
    pub owner_id: Uuid,
    pub org_tier_id: Uuid,

    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub retired_at: Option<NaiveDate>,
}

impl NewOrgOwnership {

    pub fn new(
        owner_id: Uuid,
        org_tier_id: Uuid,
    ) -> Self {
        NewOrgOwnership {
            owner_id,
            org_tier_id,
        }
    }
}
