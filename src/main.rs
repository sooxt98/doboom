#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod user;

fn main() {
    rocket::ignite()
        .mount("/", routes![user::hello])
        .launch();
}
