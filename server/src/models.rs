use crate::schema::user;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
