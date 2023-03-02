use diesel::{Connection, ConnectionResult, PgConnection};
use std::env;

pub fn establish_connection() -> () {
    let database_host = env::var("DATABASE_HOST");
    let database_port = env::var("DATABASE_PORT");
    let database_url: String;
    let pg_connection: ConnectionResult<PgConnection>;

    match database_host {
        Ok(db_host) => match database_port {
            Ok(port) => {
                database_url = format!(
                    "postgres://postgres:g8osw5TOX8OAnSXO@{}:{}/postgres",
                    db_host, port
                );
            }
            Err(e) => panic!("Error on load Database port env var, Error: {}", e),
        },
        Err(e) => panic!("Error on load Database host env var, Error: {}", e),
    }

    pg_connection = PgConnection::establish(&database_url);

    match pg_connection {
        Ok(_) => {
            println!("Sucess connect with database")
        }
        Err(err) => {
            panic!("Error on connect with data base, err: {}", err)
        }
    }
}
