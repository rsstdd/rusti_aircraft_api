#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod airplanes;
mod models;
mod pg_pool;
mod schema;

use crate::airplanes::Airplane;
use crate::rocket_contrib::json::{Json, JsonValue};
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rust 2018!"
}

#[get("/aircraft")]
fn read(connection: pg_pool::Connection) -> Json<JsonValue> {
    Json(json!(Airplane::read(&connection)))
}

#[get("/aircraft/<id>")]
fn read_id(id: i32, connection: pg_pool::Connection) -> Json<JsonValue> {
    Json(json!(Airplane::read_id(id, &connection)))
}

#[put("/aircraft/<id>", data = "<aircraft>")]
fn update(id: i32, aircraft: Json<Airplane>, connection: pg_pool::Connection) -> Json<JsonValue> {
    println!("{:?}", aircraft);

    let updated_aircraft = Airplane {
        id: Some(id),
        ..aircraft.into_inner()
    };

    Json(json!(Airplane::update(id, updated_aircraft, &connection)))
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    rocket::ignite()
        .manage(pg_pool::connect(&database_url))
        .mount("/api/", routes![index])
        .mount("/api/", routes![read, read_id, update])
        .register(catchers![not_found])
        .launch();
}
