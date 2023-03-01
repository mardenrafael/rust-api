#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod controllers;
pub mod services;
pub mod models;


use controllers::user_controller;
use services::user_service::UserService;

fn config_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![
            user_controller::create_user
        ])
        .manage(UserService::new())
}
fn main() {
    config_rocket().launch();
}
