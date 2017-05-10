#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(needless_pass_by_value))]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate lazy_static;

extern crate toml;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate rocket;
#[macro_use] extern crate rocket_contrib;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate jsonwebtoken as jwt;
extern crate base64;
extern crate time;
extern crate url;
extern crate ring;
extern crate dotenv;
extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;

mod schema;
mod models;

mod config;
mod database;

mod catchers;
mod endpoints;
mod endpoint_error;

use std::env;
use database::Db;
use endpoints::api_v1;
use endpoints::auth;

fn main() {
    let runtime_config = config::parse();

    /// Database connector
    let db_addr = env::var("DATABASE_URL").unwrap();
    let mut db = Db::new(db_addr);

    match db.init() {
        Ok(_) => (),
        Err(y) => panic!(y)
    };

    rocket::ignite()
        .mount("/", routes![
            auth::refresh_token,
            auth::google_oauth,
            auth::twitter_oauth,
            auth::facebook_oauth
        ])
        .mount("/api/v1/", routes![
            api_v1::users::get_profile,
        ])
        .catch(errors![
            catchers::not_found,
            catchers::bad_request,
            catchers::unauthorized,
            catchers::forbidden,
        ])
        .manage(runtime_config)
        .manage(db)
        .launch();
}

