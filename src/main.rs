#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod controllers;
pub mod models;
pub mod services;
pub mod utils;

use controllers::user_controller;

use services::user_service::UserService;
use utils::supa_api::SupaApi;

fn config_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/api",
            routes![
                user_controller::create_user,
                user_controller::get_all_users,
                user_controller::get_user_by_id,
                user_controller::update_user_by_id,
                user_controller::delete_user_by_id,
            ],
        )
        .manage(UserService::new(SupaApi::new()))
}

fn main() {
    dotenv::dotenv().ok();
    config_rocket().launch();
}
