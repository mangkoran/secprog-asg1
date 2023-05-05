use server::db::*;
use std::env::args;

fn main() {
    let target = args().nth(1).expect("Expected a username to get");
    let user = get_user(target.as_str());

    println!("Username: {}", user.username);
    println!("Password Hash: {}", user.password);
}
