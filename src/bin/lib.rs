pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  PgConnection::establist(&db_url)
    .unwrap_or_else(|_| panic!("Could not connect to database: {}", db_url))
}
