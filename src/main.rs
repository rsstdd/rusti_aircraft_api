#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod airplanes;
mod pg_pool;
mod schema;
use airplanes::Aircrafts;

use rocket_contrib::json::{Json, JsonValue};

use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rust 2018!"
}

#[get("/aircraft")]
fn read(connection: pg_pool::Connection) -> Json<JsonValue> {
    println!("IN READ");
    Json(json!(Aircrafts::read(&connection)))
}

#[get("/aircraft/<id>")]
fn read_id(id: i32, connection: pg_pool::Connection) -> Json<JsonValue> {
    Json(json!(Aircrafts::read_id(id, &connection)))
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    rocket::ignite()
        .manage(pg_pool::connect(&database_url))
        .mount("/api/", routes![index])
        .mount("/api/", routes![read, read_id])
        .register(catchers![not_found])
        .launch();
}
