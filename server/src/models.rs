use crate::schema::user;
use diesel::prelude::*;

#[derive(Queryable)]
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
