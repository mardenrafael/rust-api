#![feature(decl_macro)]

#[macro_use] extern crate rocket;

pub mod controllers;
pub mod services;



fn config_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![])
}
fn main() {
    config_rocket().launch();
}
