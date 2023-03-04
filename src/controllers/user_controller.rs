use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;

use crate::models::user::User;
use crate::services::user_service::UserService;

#[post("/user", format = "json", data = "<user>")]
pub fn create_user(
    user_service: State<UserService>,
    user: Json<User>,
) -> Result<Json<User>, Status> {
    let saved_user: Json<User> = user_service.create_user(user).unwrap();
    Ok(saved_user)
}

#[get("/user")]
pub fn get_all_users(_user_service: State<UserService>) -> Json<&str> {
    todo!("get user with service");
    // return Json("");
}

#[get("/user/<_id>")]
pub fn get_user_by_id(_user_service: State<UserService>, _id: i32) -> Json<&str> {
    todo!("get user by id with service");
    // return Json("");
}

pub fn update_user_by_id(_user_service: State<UserService>) -> Json<&str> {
    todo!("update user by id with service");
    // return Json("");
}

pub fn delete_user_by_id(_user_service: State<UserService>) -> Json<&str> {
    todo!("delete user by id with service");
    // return Json("");
}
