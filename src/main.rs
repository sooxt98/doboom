#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate time;
extern crate ring;
extern crate envy;
extern crate r2d2;
extern crate hyper;
extern crate rocket;
extern crate diesel;
extern crate dotenv;
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

/// This is Doboom server's runtime configurations
/// Edit ../config.env to change the settings.
#[derive(Deserialize, Debug)]
struct DoboomConfig {
    database_url: String,
    // facebook_appsecret: String,
    // twitter: String,
    // google: String,
}

fn main() {
    // Read configuration from the project root directory
    dotenv::dotenv().expect("Failed to read `.env` file");
    
    println!("📝  Printing the runtime configuration.");
    match envy::from_env::<DoboomConfig>() {
        Ok(config) => println!("{:?}", config),
        Err(reason) => println!("Couldn't get the config ({})", reason),
    };
    println!("");

    rocket::ignite()
        // authentication
        .mount("/", routes![
            //auth::jwt_auth,
            //auth::google_oauth,
            //auth::twitter_oauth,
            auth::facebook_oauth
        ])

        .mount("/test/", routes![
            user::count,
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

