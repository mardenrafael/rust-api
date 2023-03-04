use reqwest::blocking::Client;

use crate::utils::env_vars::get_env;

use super::{either::Either, http_verbs::HttpVerbs};

pub struct SupaApi {}

impl SupaApi {
    /// make request to supabase
    pub fn make_request(
        verb: HttpVerbs,
        end_point: &str,
    ) -> Result<String, Result<reqwest::Error, &str>> {
        let client: reqwest::blocking::Client = Client::new(); // Reqwest Client to provide requestBuilder
        let supa_url: String = get_env("DATABASE_HOST"); // Supabase URL
        let op_req_builder: Option<reqwest::blocking::RequestBuilder>;
        // let supa_res: Result<reqwest::blocking::Response, reqwest::Error> = client
        //     .get(supa_url.to_owned() + end_point)
        //     .query(&[("apikey", &get_env("PRIVATE_API_KEY"))])
        //     .send();

        if verb == HttpVerbs::GET {
            println!(
                "Making request to: {} \n\t Method: {}",
                supa_url.clone() + end_point,
                verb
            );

            op_req_builder = Some(client.get(supa_url.clone() + end_point));
        }

        if verb == HttpVerbs::POST {
            println!(
                "Making request to: {} \n\t Method: {}",
                supa_url.clone() + end_point,
                verb
            );
        }
        if verb == HttpVerbs::PUT {
            println!(
                "Making request to: {} \n\t Method: {}",
                supa_url.clone() + end_point,
                verb
            );
        }
        if verb == HttpVerbs::DELETE {
            println!(
                "Making request to: {} \n\t Method: {}",
                supa_url + end_point,
                verb
            );
        }

        if let Some(req_builder) = op_req_builder {
            req_builder.send();
        } else {
            Err::<reqwest::Error, &str>("Erro on build request");
        }

        match Some(op_req_builder) {
            Some(req_builder) => {}
            None => todo!(),
        }

        Ok("Return Result of request!".to_string())
        // match supa_res {
        //     Ok(res) => res.text(),
        //     Err(err) => Err(err),
        // }
    }
}
