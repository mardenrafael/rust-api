use rocket::State;
use rocket_contrib::json::Json;

use crate::models::user::User;
use crate::services::user_service::UserService;

#[post("/user", format = "json", data = "<user>")]
pub fn create_user(
    user_service: State<UserService>,
    user: Json<User>,
) -> Result<String, reqwest::Error> {
    let saved_user = user_service.create_user(user);

    match saved_user {
        Ok(sucess) => Ok(sucess),
        Err(err) => Err(err),
    }
}

#[get("/user")]
pub fn get_all_users(user_service: State<UserService>) -> Result<String, reqwest::Error> {
    user_service.get_all_users()
}

#[get("/user/<id>")]
pub fn get_user_by_id(user_service: State<UserService>, id: i8) -> Result<String, reqwest::Error> {
    user_service.get_user_by_id(id)
}

#[patch("/user/<id>", format = "json", data = "<user_data>")]
pub fn update_user_by_id(
    user_service: State<UserService>,
    id: i8,
    user_data: Json<User>,
) -> Result<String, reqwest::Error> {
    let mut user_data_with_id = user_data.0;
    user_data_with_id.id = Some(id);

    user_service.update_user_by_id(id, rocket_contrib::json::Json(user_data_with_id))
}

#[delete("/user/<id>")]
pub fn delete_user_by_id(
    user_service: State<UserService>,
    id: i8,
) -> Result<String, reqwest::Error> {
    user_service.delete_user_by_id(id)
}
