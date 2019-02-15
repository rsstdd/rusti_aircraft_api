use crate::{
    diesel::pg::PgConnection,
    diesel::r2d2::{ConnectionManager, Pool, PooledConnection},
    rocket::http::Status,
    rocket::request::{self, FromRequest},
    rocket::{Outcome, Request, State},
};
use std::ops::Deref;

type ManagedPgConn = ConnectionManager<PgConnection>;
type PgPool = Pool<ManagedPgConn>;

/// Db Connection request guard type: wrapper around r2d2 pooled connection
pub struct Connection(pub PooledConnection<ManagedPgConn>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &Connection as an &PgConnection.
impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn connect(database_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Failed to create pool.")
}
