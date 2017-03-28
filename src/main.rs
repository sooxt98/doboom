#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate time;
extern crate ring;
extern crate hyper;
extern crate rocket;
extern crate serde_json;
extern crate rustc_serialize;
extern crate jsonwebtoken as jwt;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod user;
mod auth;
mod catchers;

fn main() {
    rocket::ignite()
        .mount("/", routes![user::hello])

        // authentication
        .mount("/", routes![
            auth::facebook_oauth
        ])

        .catch(errors![
            catchers::not_found,
            catchers::bad_request,
            catchers::unauthorized,
        ])

        .launch();
}
