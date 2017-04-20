#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate url;
extern crate time;
extern crate ring;
extern crate envy;
extern crate r2d2;
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

/// Funtionality
mod db;
mod auth;
mod catchers;

/// API definitions
mod user;
// mod post;
// mod product;
// mod organization;

fn main() {
    // Read configuration from the project root directory
    dotenv::dotenv().expect("Failed to read `.env` file.");

    rocket::ignite()
        // authentication
        .mount("/", routes![
            //auth::jwt_auth,
            auth::refresh_token,
            auth::google_oauth,
            auth::twitter_oauth,
            auth::facebook_oauth
        ])

        // user related API
        .mount("/v1", routes![
            user::hello,
        ])

        .catch(errors![
            catchers::not_found,
            catchers::bad_request,
            catchers::unauthorized,
            catchers::forbidden,
        ])

        .launch();
}

