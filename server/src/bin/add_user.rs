use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use server::db::*;
use server::utils::*;

fn main() {
    let name = Alphanumeric.sample_string(&mut thread_rng(), 8);
    let password = "bar";
    let user = add_user(name.as_str(), encrypt_password(password).as_str());

    println!("Added username: {}", user.username);
    println!("Added password: {}", user.password);
}
