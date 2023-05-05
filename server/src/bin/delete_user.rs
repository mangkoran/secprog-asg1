use server::db::*;
use std::env::args;

fn main() {
    let target = args().nth(1).expect("Expected a username to delete");

    delete_user(target.as_str());
}
