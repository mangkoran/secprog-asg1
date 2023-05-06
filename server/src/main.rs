#[macro_use]
extern crate rocket;

// use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::{get, routes};
// use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins};
use std::error::Error;

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

#[post("/users/update", format = "json", data = "<user>")]
fn update(user: Json<User>) -> Json<User> {
    println!("Received username: {}", user.username);
    println!("Received password: {}", user.password);

    let result =
        update_user(user.username.as_str(), user.password.as_str()).expect("Error updating user");

    Json(result)
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index, get_all, create, login])
// }

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let cors = rocket_cors::CorsOptions {
    //     allowed_origins: AllowedOrigins::all(),
    //     allowed_methods: vec![Method::Get, Method::Post]
    //         .into_iter()
    //         .map(From::from)
    //         .collect(),
    //     // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
    //     allowed_headers: AllowedHeaders::all(),
    //     allow_credentials: true,
    //     ..Default::default()
    // }
    // .to_cors()?;
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    let _ = rocket::build()
        .mount("/", routes![index, get_all, create, login, update])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
