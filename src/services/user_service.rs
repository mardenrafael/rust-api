use rocket_contrib::json::Json;

use crate::models::user::User;

pub struct UserService {}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_user(&self, user_to_save: Json<User>) -> Result<Json<User>, ()> {
        println!("Make some validations and business logic");
        Ok(user_to_save)
    }

    /// Get all users
    pub fn get_all_users() {
        todo!("get user not implemented")
    }

    /// Get user by id
    pub fn get_user_by_id() {
        todo!("get user by id not implemented")
    }

    /// Delete user locate by id
    pub fn delete_user_by_id() {
        todo!("get user by id not implemented")
    }

    /// Update user localate by id
    pub fn update_user_by_id() {
        todo!("update user by id not implemented")
    }
}

impl Default for UserService {
    fn default() -> Self {
        Self::new()
    }
}
