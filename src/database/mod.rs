use std::env;
use rocket::Request;
use rocket::http::Status;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::Outcome::{Success, Failure};
use rocket::request::{Outcome, FromRequest};
use r2d2::{Pool, Config, PooledConnection, GetTimeout};

pub mod schema;
pub mod models;

lazy_static! {
    pub static ref DB_POOL:
        Pool<ConnectionManager<PgConnection>> = establish_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    /// Function that returns our DB struct if successful or an error
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(y) => Failure((Status::InternalServerError, y)),
        }
    }
}

// This will create a pool of database connection. Technically it's a
// thread pool of database connections to postgres in case we need to
// generate a connection each time a request received, which is inefficient.
pub fn establish_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = Config::default();
    let connManager = ConnectionManager::<PgConnection>::new(database_url);
    
    Pool::new(config, connManager).expect("Failed to create pool of database connection")
}
