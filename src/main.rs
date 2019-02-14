#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]

// extern crate bcrypt;
// extern crate chrono;
#[macro_use]
extern crate diesel;
// extern crate diesel_migrations;
extern crate dotenv;
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod airplanes;
mod pg_pool;
use airplanes::Airplanes;

use rocket_contrib::json::{Json, JsonValue};

use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rust 2018!"
}

#[get("/api/aircraft")]
fn read(connection: pg_pool::Connection) -> Json<Value> {
    Json(Airplanes::read(&connection))
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resourcce was not found."
    })
}

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    rocket::ignite()
        .manage(pg_pool::connect(&database_url))
        .mount("/api/", routes![index])
        .mount("/api/aircraft", routes![read])
        .register(catchers![not_found])
        .launch();
}
