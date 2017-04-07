use rocket::Request;
use rocket::http::Status;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::Outcome::{Success, Failure};
use rocket::request::{Outcome, FromRequest};
use r2d2::{Pool, PooledConnection, GetTimeout};

use database::establish_db_pool;

lazy_static! {
    pub static ref DB_POOL:
        Pool<ConnectionManager<PgConnection>> = establish_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(why) => Failure((Status::InternalServerError, why)),
        }
    }
}

