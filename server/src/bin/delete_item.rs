use diesel::prelude::*;
use server::*;
use std::env::args;

pub fn delete_item(item_name: &str) {
    use server::schema::item::dsl::*;

    let pattern = format!("%{}%", item_name);
    let conn = &mut establish_connection();
    let num_deleted = diesel::delete(item.filter(name.like(pattern)))
        .execute(conn)
        .expect("Error deleting item");

    println!("Deleted {} posts", num_deleted);
}

fn main() {
    let target = args().nth(1).expect("Expected a target to match against");

    delete_item(target.as_str());
}
