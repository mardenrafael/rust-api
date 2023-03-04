use diesel::{Connection, PgConnection};

use crate::utils::env_vars::get_env;

pub fn establish_connection() -> PgConnection {
    // let database_host: String = get_env("DATABASE_HOST");
    // let database_name: String = get_env("DATABASE_NAME");
    let database_url = get_env("DATABASE_URL");
    // let database_url: String = format!("postgres://{}/{}", database_host, database_name);

    PgConnection::establish(&database_url).unwrap_or_else(|e| panic!("Error on connect: {}", e))
    // println!("{}", &database_url);
}
