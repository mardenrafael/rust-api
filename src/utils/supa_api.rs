use std::collections::HashMap;

use reqwest::blocking::Client;

use crate::utils::env_vars::get_env;

use super::http_verbs::HttpVerbs;

pub struct SupaApi {}

impl SupaApi {
    pub fn make_request(
        verb: HttpVerbs,
        end_point: &str,
        query: Option<&str>,
        body: Option<HashMap<String, String>>,
    ) -> Result<String, reqwest::Error> {
        let client: reqwest::blocking::Client = Client::new();

        let mut supa_host: String = Default::default();
        get_env("DATABASE_HOST", &mut supa_host);

        let mut private_key: String = Default::default();
        get_env("PRIVATE_API_KEY", &mut private_key);

        let query_params: &str;
        let request_body: HashMap<String, String>;

        query_params = match query {
            Some(matched_query_params) => matched_query_params,
            None => "",
        };

        // TODO: Use Json from serde for post, put and patch request
        match body {
            Some(body_hashmap) => request_body = body_hashmap,
            None => request_body = HashMap::new(),
        }

        let supa_url: String = supa_host.to_owned() + end_point + query_params;
        println!("{}", supa_url);
        let mut op_req_builder: Option<reqwest::blocking::RequestBuilder> = None;

        match verb {
            HttpVerbs::GET => {
                op_req_builder = Some(client.get(&supa_url).query(&[("apikey", private_key)]))
            }
            HttpVerbs::POST => {
                op_req_builder = Some(
                    client
                        .post(&supa_url)
                        .query(&[("apikey", private_key)])
                        .json(&request_body),
                )
            }
            HttpVerbs::PUT => {
                op_req_builder = Some(client.put(&supa_url).query(&[("apikey", private_key)]))
            }
            HttpVerbs::DELETE => {
                op_req_builder = Some(client.delete(&supa_url).query(&[("apikey", private_key)]))
            }
            HttpVerbs::PATCH => {
                op_req_builder = Some(
                    client
                        .patch(&supa_url)
                        .query(&[("apikey", private_key)])
                        .json(&request_body),
                )
            }
        }

        let response = match op_req_builder {
            Some(request) => request.send(),
            None => {
                // TODO: Trater erro de forma correta
                panic!("request not build proper")
            }
        };

        match response {
            Ok(data) => match data.text() {
                Ok(text) => {
                    if text.is_empty() {
                        Ok("No content response".to_string())
                    } else {
                        Ok(text)
                    }
                }
                Err(err) => Err(err),
            },
            // TODO: tratar erros da forma correta
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
