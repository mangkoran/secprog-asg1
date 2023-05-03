use diesel::prelude::*;
use encrypt_password::encrypt_password;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use server::models::*;
use server::*;

mod encrypt_password;

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
    let rand_string: String = Alphanumeric.sample_string(&mut thread_rng(), 8);
    let password = "bar";

    println!("Name: {rand_string}");
    println!("Password: {password}");

    add_item(rand_string.as_str(), encrypt_password(password).as_str());
}
