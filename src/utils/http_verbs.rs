use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum HttpVerbs {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl Display for HttpVerbs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            HttpVerbs::GET => {
                writeln!(f, "GET")
            }
            HttpVerbs::POST => {
                writeln!(f, "POST")
            }
            HttpVerbs::PUT => {
                writeln!(f, "PUT")
            }
            HttpVerbs::DELETE => {
                writeln!(f, "DELETE")
            }
            HttpVerbs::PATCH => {
                writeln!(f, "PATCH")
            }
        }
    }
}
