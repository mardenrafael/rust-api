#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

pub mod controllers;
pub mod database;
pub mod models;
pub mod services;
pub mod utils;

use controllers::user_controller;
use services::user_service::UserService;
use utils::supa_api::SupaApi;

use utils::http_verbs::HttpVerbs;

fn _config_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![user_controller::create_user])
        .manage(UserService::new())
}

fn main() {
    dotenv::dotenv().ok();

    let res = SupaApi::make_request(HttpVerbs::DELETE, "/rest/v1/users")
        .unwrap_or_else(|e| panic!("{}", e));
    println!("{}", res);
}
