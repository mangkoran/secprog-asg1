mod encrypt_password;

use diesel::prelude::*;
use encrypt_password::encrypt_password;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use server::models::*;
use server::*;

pub fn add_item(name: &str, password: &str) -> Item {
    use crate::schema::item;

    let conn = &mut establish_connection();
    let new_item = NewItem { name, password };

    diesel::insert_into(item::table)
        .values(&new_item)
        // .execute(conn)
        .get_result(conn)
        .expect("Error saving new item")
}

fn main() {
    let name = Alphanumeric.sample_string(&mut thread_rng(), 8);
    let password = "bar";
    let item = add_item(name.as_str(), encrypt_password(password).as_str());

    println!("Added name: {}", item.name);
    println!("Added password: {}", item.password);
}
