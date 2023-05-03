use diesel::prelude::*;
use server::models::*;
use server::*;

pub fn add_item(name: &str, password: &str) -> usize {
    use crate::schema::item;

    let conn = &mut establish_connection();

    let new_item = NewItem { name, password };

    diesel::insert_into(item::table)
        .values(&new_item)
        .execute(conn)
        .expect("Error saving new item")
}

fn main() {
    add_item("foo", "bar");
}
