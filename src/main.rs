#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)]
mod tests;

use rocket::{get, routes};
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
fn hello() -> &'static str {
    "Hello, Rust 2018!"
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
