use std::env;

pub fn get_env(var: &str) -> String {
    match env::var(var) {
        Ok(env_var) => env_var,
        Err(e) => {
            panic!("Env var not found or defined, Error: {}", e);
        }
    }
}
