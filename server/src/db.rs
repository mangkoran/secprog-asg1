use crate::models::*;
use crate::utils;
use crate::*;

use diesel::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;

use utils::encrypt_password;

pub fn add_item(name: &str, password: &str) -> Item {
    use crate::schema::item;

    let conn = &mut establish_connection();
    let new_item = NewItem { name, password };

    diesel::insert_into(item::table)
        .values(&new_item)
        // .execute(conn)
        .get_result(conn)
        .expect("Error adding item")
}

pub fn get_item() -> Vec<Item> {
    use crate::schema::item::dsl::*;

    let conn = &mut establish_connection();

    item.load::<Item>(conn).expect("Error loading items")
}

pub fn delete_item(item_name: &str) {
    use server::schema::item::dsl::*;

    let pattern = format!("%{}%", item_name);
    let conn = &mut establish_connection();
    let num_deleted = diesel::delete(item.filter(name.like(pattern)))
        .execute(conn)
        .expect("Error deleting item");
}

fn main() {
    unimplemented!();
}
