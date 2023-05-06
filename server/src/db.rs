use crate::models::*;
use crate::utils::*;
use crate::*;

use diesel::prelude::*;

pub fn add_user(username: &str, password: &str) -> Result<User, diesel::result::Error> {
    use crate::schema::user;

    let conn = &mut establish_connection();
    let hashed_password = encrypt_password(password);
    let new_user = NewUser {
        username,
        password: hashed_password.as_str(),
    };

    diesel::insert_into(user::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user(username: &str) -> Result<User, diesel::result::Error> {
    use crate::schema::user::dsl;

    let conn = &mut establish_connection();

    dsl::user.find(username).first::<User>(conn)
}

pub fn get_users() -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::user::dsl;

    let conn = &mut establish_connection();

    dsl::user.load::<User>(conn)
}

pub fn update_user(username: &str, password: &str) -> Result<User, diesel::result::Error> {
    use crate::schema::user::dsl;

    let conn = &mut establish_connection();
    let new_password = encrypt_password(password);

    // dsl::user.find(username).first::<User>(conn)
    diesel::update(dsl::user.find(username))
        .set(dsl::password.eq(new_password.as_str()))
        .get_result(conn)
}

pub fn delete_user(username: &str) -> Result<User, diesel::result::Error> {
    use crate::schema::user::dsl;

    let conn = &mut establish_connection();

    diesel::delete(dsl::user.filter(dsl::username.eq(username))).get_result(conn)
}
