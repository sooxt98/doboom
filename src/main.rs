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

use std::env as std_env;
use std::str::FromStr;

mod schema;
mod models;

mod config;
mod env;
mod db;

mod catchers;
mod endpoints;
mod endpoint_error;

use env::Env;
use config::DbConfig;
use db::Db;
use endpoints::api_v1;
use endpoints::auth;

fn main() {
    let env_str = &std_env::var("DOBOOM_ENV").unwrap_or_else(|_| "development".to_owned());
    let env = Env::from_str(env_str).unwrap_or_default();
    let db_config = DbConfig::load(&env).expect("Error loading DB configuration");
    let mut db = Db::new(db_config);

    match db.init() {
        Ok(_) => {
            rocket::ignite()
                .mount("/", routes![
                       // auth::jwt_auth,
                       auth::refresh_token,
                       auth::google_oauth,
                       auth::twitter_oauth,
                       auth::facebook_oauth
                ])
                
                //.mount("/api/v1", routes![
                //       api_v1::users::hello,
                //])

                .catch(errors![
                       catchers::not_found,
                       catchers::bad_request,
                       catchers::unauthorized,
                       catchers::forbidden,
                ])

                .manage(db)
                .launch()
        }
        Err(reason) => println!("Db initialization error: {}", reason),
    };
}

