use server::db::*;
use std::env::args;

fn main() {
    let target = args().nth(1).expect("Expected a username to delete");

    let result = delete_user(target.as_str()).expect("Error deleting user");

    println!("Deleted username: {}", result.username);
}
