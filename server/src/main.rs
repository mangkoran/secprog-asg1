#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use server::{db::*, models::User};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/user/create", format = "json", data = "<user>")]
fn user(user: Json<User>) {
    println!("Received username: {}", user.username);
    println!("Received password: {}", user.password);

    let added_user = add_user(user.username.as_str(), user.password.as_str());

    println!("Added username: {}", added_user.username);
    println!("Added password: {}", added_user.password);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, user])
}
