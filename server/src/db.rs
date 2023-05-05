use crate::models::*;
use crate::utils::*;
use crate::*;

use diesel::prelude::*;

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

pub fn get_user(username: &str) -> User {
    use crate::schema::user::dsl;

    let conn = &mut establish_connection();

    dsl::user
        .find(username)
        .load::<User>(conn)
        .expect("Error")
        .into_iter()
        .nth(0)
        .expect("Error getting user")
}

pub fn get_users() -> Vec<User> {
    use crate::schema::user::dsl;

    let conn = &mut establish_connection();

    dsl::user.load::<User>(conn).expect("Error getting users")
}

pub fn delete_user(username: &str) -> User {
    use crate::schema::user::dsl;

    let conn = &mut establish_connection();

    diesel::delete(dsl::user.filter(dsl::username.eq(username)))
        .get_result(conn)
        .expect("Error deleting user")
}
