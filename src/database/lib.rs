extern crate r2d2;
extern crate dotenv;
extern crate r2d2_diesel;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

use std::env;
use dotenv::dotenv;
use r2d2::{Pool, Config};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub mod schema;
pub mod models;

// This will create a pool of database connection. Technically it's a
// thread pool of database connections to postgres in case we need to
// generate a connection each time a request received, which is inefficient.
pub fn establish_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    Pool::new(config, manager).expect("Failed to create pool of database connection")
}

