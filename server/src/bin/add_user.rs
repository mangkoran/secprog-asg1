use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use server::db::*;
use server::utils::*;

fn main() {
    let name = Alphanumeric.sample_string(&mut thread_rng(), 8);
    let password = "bar";
    let result =
        add_user(name.as_str(), encrypt_password(password).as_str()).expect("Error adding user");

    println!("Added username: {}", result.username);
    println!("Added password: {}", result.password);
}
