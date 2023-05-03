use diesel::prelude::*;
use server::models::*;
use server::*;

pub fn get_item() -> Vec<Item> {
    use server::schema::item::dsl::*;

    let conn = &mut establish_connection();

    item.load::<Item>(conn).expect("Error loading items")
}

fn main() {
    let binding = get_item();
    let items = binding.iter().enumerate();
    let items_len = items.len();

    println!("Displaying {} items", items_len);
    println!("");

    for (i, item) in items {
        println!("Item {}", i + 1);
        println!("Name: {}", item.name);
        println!("Hashed Password: {}", item.password);

        if i + 1 != items_len {
            println!();
        }
    }
}
