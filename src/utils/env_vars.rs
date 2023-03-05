use std::env;

pub fn get_env(var: &str, target: &mut String) -> () {
    match env::var(var) {
        Ok(env_var) => env_var.clone_into(target),
        Err(e) => {
            // TODO: Tratar erro de forma correta
            panic!("Env var not found or defined, Error: {}", e);
        }
    }
}
