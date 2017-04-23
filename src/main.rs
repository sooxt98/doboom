#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate url;
extern crate time;
extern crate ring;
extern crate envy;
extern crate r2d2;
extern crate toml;
extern crate hyper;
extern crate rocket;
extern crate diesel;
extern crate dotenv;
extern crate futures;
extern crate hyper_tls;
extern crate tokio_core;
extern crate serde_json;
extern crate r2d2_diesel;
extern crate rustc_serialize;
extern crate jsonwebtoken as jwt;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

extern crate database;

use std::env as std_env;
use std::str::FromStr;

/// Funtionality
mod db;
mod env;
mod auth;
mod config;
mod catchers;

use env::Env;
use config::DbConfig;
use db::DB;

/// API definitions
mod endpoints;

use endpoints::api_v1;

fn main() {
    // Read configuration from the project root directory
    dotenv::dotenv().expect("Failed to read `.env` file.");

    let env_str = &std_env::var("DOBOOM_ENV").unwrap_or_else(|_| "development".to_owned());
    let env = Env::from_str(env_str).unwrap_or_default();
    let db_config = DbConfig::load(&env).expect("Error loading DB configuration");

    let mut db = Db::new(db_config);

    match db.init() {
        Ok(_) => {
            rocket::ignite()
                // authentication
                .mount("/", routes![
                    auth::jwt_auth,
                    auth::refresh_token,
                    auth::google_oauth,
                    auth::twitter_oauth,
                    auth::facebook_oauth
                ])

                // functions
                .mount("/api/v1", routes![
                    api_v1::users::hello,
                ])

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

