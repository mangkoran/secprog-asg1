#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use server::{db::*, models::User, utils::check_user};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn get_all() -> Json<Vec<User>> {
    let result = get_users().expect("Error loading users");

    Json(result)
}

#[post("/users/create", format = "json", data = "<user>")]
fn create(user: Json<User>) -> Json<User> {
    println!("Received username: {}", user.username);
    println!("Received password: {}", user.password);

    let result =
        add_user(user.username.as_str(), user.password.as_str()).expect("Error adding user");

    Json(result)
}

#[post("/users/login", format = "json", data = "<user>")]
fn login(user: Json<User>) -> Json<User> {
    println!("Received username: {}", user.username);
    println!("Received password: {}", user.password);

    let result =
        check_user(user.username.as_str(), user.password.as_str()).expect("Error loading user");

    Json(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_all, create, login])
}
