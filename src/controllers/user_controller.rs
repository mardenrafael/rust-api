use rocket::{State, response::content::Json};

use crate::services::user_service::UserService;


struct UserController {
}

impl UserController {
    

    pub fn get_all_users(user_service: State<UserService>) -> Json<&str> {
        todo!("get user with service");
        return Json("");
    }

    pub fn get_user_by_id(user_service: State<UserService>) -> Json<&str> {
        todo!("get user by id with service");
        return Json("");
    }

    pub fn update_user_by_id(user_service: State<UserService>) -> Json<&str> {
        todo!("update user by id with service");
        return Json("");
    }

    pub fn delete_user_by_id(user_service: State<UserService>) -> Json<&str> {
        todo!("delete user by id with service");
        return Json("");
    }
}