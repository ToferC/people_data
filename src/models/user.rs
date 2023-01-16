// Modeling from: https://github.com/clifinger/canduma/blob/master/src/user/handler.rs

use uuid::Uuid;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use inflector::Inflector;

use crate::schema::users;
use crate::database;
use crate::errors::CustomError;

use shrinkwraprs::Shrinkwrap;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::{QueryDsl};

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Associations, Identifiable, AsChangeset, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub hash: Vec<u8>,
    pub salt: String,
    pub email: String,
    pub user_name: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
    pub role: String,
    pub validated: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub hash: Vec<u8>,
    pub salt: String,
    pub email: String,
    pub user_name: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
    pub role: String,
    pub validated: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimUser {
    pub user_name: String,
    pub email: String,
    pub slug: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserData {
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub validated: bool,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct LoggedUser(pub Option<SlimUser>);

impl From<SlimUser> for LoggedUser {
    fn from(slim_user: SlimUser) -> Self {
        LoggedUser(Some(slim_user))
    }
}

impl From<UserData> for InsertableUser {
    fn from(user_data: UserData) -> Self {
        let UserData {
            user_name,
            email,
            password,
            role,
            validated,
            ..
        } = user_data;

        let salt = make_salt();
        let hash = make_hash(&password, &salt).as_bytes().to_vec();
        
        Self {
            user_name: user_name.clone(),
            slug: user_name.clone().to_snake_case(),
            email,
            hash,
            created_at: chrono::Local::now().naive_local(),
            salt,
            role,
            validated,
        }
    }
}

impl User {
    pub fn create(user_data: UserData) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let insertable_user = InsertableUser::from(user_data.clone());
        let user = diesel::insert_into(users::table)
            .values(insertable_user)
            .on_conflict(users::id)
            .do_update()
            .set((
                users::role.eq(&user_data.role),
                users::validated.eq(&user_data.validated),
            ))
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = database::connection()?;
        let users = users::table.load::<User>(&conn)?;
        Ok(users)
    }

    pub fn find_admins() -> Result<Vec<Self>, CustomError> {
        let conn = database::connection()?;
        let users = users::table
            .filter(users::role.eq("admin"))
            .load::<User>(&conn)?;
        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let user = users::table.filter(users::id.eq(id)).first(&conn)?;
        Ok(user)
    }

    pub fn find_from_email(email: &String) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let user: User = users::table.filter(users::email.eq(email)).first(&conn)?;
        Ok(user)
    }

    pub fn find_slim_from_email(email: &String) -> Result<SlimUser, CustomError> {
        let conn = database::connection()?;
        let user: User = users::table.filter(users::email.eq(email)).first(&conn)?;

        let sl = SlimUser::from(user);
        Ok(sl)
    }

    pub fn find_from_slug(slug: &String) -> Result<User, CustomError> {
        let conn = database::connection()?;
        let user: User = users::table.filter(users::slug.eq(slug)).first(&conn)?;
        Ok(user)
    }

    pub fn find_id_from_slug(slug: &String) -> Result<Uuid, CustomError> {
        let conn = database::connection()?;
        let id: Uuid = users::table.select(users::id).filter(users::slug.eq(slug)).first(&conn)?;
        Ok(id)
    }

    pub fn find_slim_from_slug(slug: &String) -> Result<SlimUser, CustomError> {
        let conn = database::connection()?;
        let user: User = users::table.filter(users::slug.eq(slug)).first(&conn)?;
        Ok(SlimUser::from(user))
    }

    pub fn find_from_user_name(user_name: &String) -> Result<SlimUser, CustomError> {
        let conn = database::connection()?;
        let user: User = users::table.filter(users::user_name.eq(user_name)).first(&conn)?;
        let sl = SlimUser::from(user);
        Ok(sl)
    }

    pub fn update(user: User) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let user = diesel::update(users::table)
            .filter(users::id.eq(user.id))
            .set(user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn update_password(user_id: Uuid, password: &String) -> Result<Self, CustomError> {
        let conn = database::connection()?;

        let salt = make_salt();

        let user = diesel::update(users::table)
            .filter(users::id.eq(user_id))
            .set((
                users::hash.eq(make_hash(&password, &salt).as_bytes().to_vec()),
                users::salt.eq(salt),
            ))
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let conn = database::connection()?;
        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&conn)?;
        Ok(res)
    }

    pub fn dummy() -> Self {
        User {
            id: Uuid::new_v4(),
            hash: Vec::new(),
            salt: "".to_string(),
            email: "".to_string(),
            user_name: "dummy".to_string(),
            slug: "".to_string(),
            created_at: NaiveDateTime::from_timestamp(1_000_000_000, 0),
            role: "".to_string(),
            validated: false,
        }
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            user_name,
            email,
            role,
            slug,
            ..
        } = user;

        Self {
            user_name,
            email,
            role,
            slug,
        }
    }
}

// Utility Functions
pub fn make_salt() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 128;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

pub fn make_hash(password: &str, salt: &str) -> String {
    let config = argon2::Config::default();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}

pub fn verify(user: &User, password: &str) -> bool {
    let User {hash, salt, ..} = user;

    make_hash(password, salt).as_bytes().to_vec() == *hash
}

pub fn has_role(user: &LoggedUser, role: &str) -> Result<bool, CustomError> {
    match user.0 {
        Some(ref user) if user.role == role => Ok(true),
        _ => Err(CustomError::new(002, "Role not present".to_string())),
    }
}