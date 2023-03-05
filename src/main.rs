#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

pub mod controllers;
pub mod models;
pub mod services;
pub mod utils;

use controllers::user_controller;
use services::user_service::UserService;
use utils::supa_api::SupaApi;

use utils::http_verbs::HttpVerbs;

fn config_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![user_controller::create_user])
        .manage(UserService::new())
}

fn main() {
    dotenv::dotenv().ok();

    // config_rocket().launch();
    let res = SupaApi::make_request(HttpVerbs::GET, "rest/v1/Users", Some("?id=eq.2"), None)
        .unwrap_or_else(|e| -> _ { panic!("{}", e) });
    println!("{}", res);
}
