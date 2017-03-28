use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod schema;

fn establish_conn() -> pgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
