use crate::models::*;
use crate::utils;
use crate::*;

use diesel::prelude::*;
// use rand::distributions::{Alphanumeric, DistString};
// use rand::thread_rng;

use utils::encrypt_password;

pub fn add_user(username: &str, password: &str) -> User {
    use crate::schema::user;

    let conn = &mut establish_connection();
    let hashed_password = encrypt_password(password);
    let new_user = NewUser {
        username,
        password: hashed_password.as_str(),
    };

    diesel::insert_into(user::table)
        .values(&new_user)
        // .execute(conn)
        .get_result(conn)
        .expect("Error adding user")
}

pub fn get_user() -> Vec<User> {
    use crate::schema::user::dsl::*;

    let conn = &mut establish_connection();

    user.load::<User>(conn).expect("Error loading users")
}

pub fn delete_user(username: &str) {
    use crate::schema::user::dsl;

    let pattern = format!("%{}%", username);
    let conn = &mut establish_connection();
    let num_deleted = diesel::delete(dsl::user.filter(dsl::username.like(pattern)))
        .execute(conn)
        .expect("Error deleting user");
}

fn main() {
    unimplemented!();
}
