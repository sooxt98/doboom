#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;

use rocket::Request;
use rocket_contrib::{JSON, Value};

mod user;

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
        .catch(errors![not_found])
        .launch();
}
