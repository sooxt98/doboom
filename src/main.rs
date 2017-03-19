#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate time;
extern crate rustc_serialize;
extern crate jsonwebtoken as jwt;

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

use rocket::Request;
use rocket_contrib::{JSON, Value};

mod user;
mod auth;

#[error(404)]
fn not_found(req: &Request) -> JSON<Value> {
    print!("{:?}", req.content_type());
    let resp = match req.content_type() {
        // Check if it's application/json typed
        Some(ref ctxt) if !ctxt.is_json() => {
            json!({
                "success": false,
                "message": format!("Sorry we only supports JSON requests, not '{}'.", ctxt)
            })
        },
        _ => {
            json!({
                "success": false,
                "message": format!("'{}' is an invalid URL.", req.uri())
            })
        }
    };
    JSON(resp)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![user::hello])

        // authentication
        .mount("/", routes![
            auth::facebook_oauth
        ])

        .catch(errors![not_found])
        .launch();
}
