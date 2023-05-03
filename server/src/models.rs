use crate::schema::item;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Item {
    pub name: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = item)]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub password: &'a str,
}
