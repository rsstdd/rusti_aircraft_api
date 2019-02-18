pub mod auth;
pub mod model;
pub mod schema;

use self::auth::crypto::sha2::Sha256;
use self::auth::jwt::{Header, Registered, Token};
use self::auth::ApiKey;
use self::model::User;
use crate::pg_pool;
use crate::rocket_contrib::json::{Json, JsonValue};
use rocket::request::{self, FromRequest, Request};
use rocket::{self, http::Status};

#[post("/", data = "<user>")]
fn create(user: Json<User>, connection: pg_pool::Connection) -> Result<Json<User>, Status> {
    let insert_user = User {
        id: None,
        ..user.into_inner()
    };

    User::create(insert_user, &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/info")]
fn info(key: ApiKey) -> Json<JsonValue> {
    Json(json!(
        {
            "success": true,
            "message": key.0
        }
    ))
}

#[get("/info", rank = 2)]
fn info_error() -> Json<JsonValue> {
    Json(json!(
        {
            "success": false,
            "message": "Not Authorized"
        }
    ))
}

#[get("/")]
fn read(_key: ApiKey, connection: pg_pool::Connection) -> Result<Json<JsonValue>, Status> {
    User::read(0, &connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

#[get("/", rank = 2)]
fn read_error() -> Json<JsonValue> {
    Json(json!(
        {
            "success": false,
            "message": "Not Authorized"
        }
    ))
}

#[get("/<id>")]
fn read_one(id: i32, connection: pg_pool::Connection) -> Result<Json<JsonValue>, Status> {
    User::read(id, &connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

#[put("/<id>")]
fn update(id: i32, user: Json<User>, connection: pg_pool::Connection) -> Json<JsonValue> {
    let updated_user = User {
        id: Some(id),
        ..user.into_inner()
    };
    Json(json!({
        "success": User::update(id, updated_user, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: pg_pool::Connection) -> Json<JsonValue> {
    Json(json!({ "success": User::delete(id, &connection) }))
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/login", data = "<credentials>")]
fn login(
    credentials: Json<Credentials>,
    connection: pg_pool::Connection,
) -> Result<Json<JsonValue>, Status> {
    let header: Header = Default::default();
    let username = credentials.username.to_string();
    let password = credentials.password.to_string();

    match User::by_username_and_password(username, password, &connection) {
        None => Err(Status::NotFound),
        Some(user) => {
            let claims = Registered {
                sub: Some(user.first_name.into()),
                ..Default::default()
            };
            let token = Token::new(header, claims);

            token
                .signed(b"secret_key", Sha256::new())
                .map(|message| {
                    Json(json!({
                        "success": true, "token": message
                    }))
                })
                .map_err(|_| Status::InternalServerError)
        }
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount(
            "/user",
            routes![read, read_error, read_one, create, update, delete, info, info_error],
        )
        .mount("/auth", routes![login])
}
