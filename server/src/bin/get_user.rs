use server::db::*;
use std::env::args;

fn main() {
    let target = args().nth(1).expect("Expected a username to get");
    let result = get_user(target.as_str()).expect("Error getting user");

    println!("Username: {}", result.username);
    println!("Password Hash: {}", result.password);
}
