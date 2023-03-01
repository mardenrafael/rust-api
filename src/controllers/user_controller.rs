use rocket::State;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::models::user::User;
use crate::services::user_service::UserService;

#[post("/user", data = "<user>")]
pub fn create_user(user_service: State<UserService>, user: Json<User>) -> Result<Json<User>, Status> {
    let saved_user: Json<User> = user_service.create_user(user).unwrap();
    println!("Connect with DB");
    return Ok(saved_user);
}

#[get("/user")]
pub fn get_all_users(_user_service: State<UserService>) -> Json<&str> {
    todo!("get user with service");
    return Json("");
}

#[get("/user/<id>")]
pub fn get_user_by_id(_user_service: State<UserService>, id: i32) -> Json<&str> {
    todo!("get user by id with service");
    return Json("");
}

pub fn update_user_by_id(_user_service: State<UserService>) -> Json<&str> {
    todo!("update user by id with service");
    return Json("");
}

pub fn delete_user_by_id(_user_service: State<UserService>) -> Json<&str> {
    todo!("delete user by id with service");
    return Json("");
}