use db::DB;
use diesel::prelude::*;
use database::models::clicks::Count;
use rocket_contrib::{JSON, Value};
use jwt::{encode, decode, Header, Algorithm};

#[derive(Deserialize, Serialize)]
pub struct Ret {
    pub count: i32,
}

#[get("/click")]
fn count(db: DB) -> JSON<Ret> {
    use database::schema::counts::dsl::*;
    let res = counts.first::<Count>(db.conn())
        .expect("Error loading data");

    JSON(Ret {
        count: res.clicks
    })
}

#[get("/")]
fn hello() -> &'static str {
    "hello doboom"
}

