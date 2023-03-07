use rocket_contrib::json::Json;

use crate::{
    models::user::User,
    utils::{http_verbs::HttpVerbs, supa_api::SupaApi},
};

pub struct UserService<'a> {
    supa_api: SupaApi,
    end_point: &'a str,
}

impl UserService<'_> {
    pub fn new(supa_api: SupaApi) -> Self {
        Self {
            supa_api,
            end_point: "rest/v1/Users",
        }
    }

    pub fn create_user(&self, user_to_save: Json<User>) -> Result<String, reqwest::Error> {
        let result =
            self.supa_api
                .make_request(HttpVerbs::POST, self.end_point, None, Some(user_to_save));

        match result {
            Ok(sucess_result) => Ok(sucess_result),
            Err(err) => Err(err),
        }
    }

    pub fn get_all_users(&self) -> Result<String, reqwest::Error> {
        let result = self
            .supa_api
            .make_request(HttpVerbs::GET, self.end_point, None, None);

        match result {
            Ok(sucess) => Ok(sucess),
            Err(err) => Err(err),
        }
    }

    pub fn get_user_by_id(&self, user_id: i8) -> Result<String, reqwest::Error> {
        let query: &str = &("?id=eq.".to_owned() + &user_id.to_string());
        let result = self
            .supa_api
            .make_request(HttpVerbs::GET, self.end_point, Some(query), None);

        match result {
            Ok(sucess) => Ok(sucess),
            Err(err) => Err(err),
        }
    }

    pub fn delete_user_by_id(&self, user_id: i8) -> Result<String, reqwest::Error> {
        let query: &str = &("?id=eq.".to_owned() + &user_id.to_string());
        let result =
            self.supa_api
                .make_request(HttpVerbs::DELETE, self.end_point, Some(query), None);

        match result {
            Ok(sucess) => Ok(sucess),
            Err(err) => Err(err),
        }
    }

    pub fn update_user_by_id(
        &self,
        user_id: i8,
        user_data: Json<User>,
    ) -> Result<String, reqwest::Error> {
        let query: &str = &("?id=eq.".to_owned() + &user_id.to_string());
        let result = self.supa_api.make_request(
            HttpVerbs::PATCH,
            self.end_point,
            Some(query),
            Some(user_data),
        );

        match result {
            Ok(sucess) => Ok(sucess),
            Err(err) => Err(err),
        }
    }
}
