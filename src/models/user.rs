use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i8>,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "id: {:#?}, \n name: {:#?}, \n email {:#?} \n\t",
            self.id, self.name, self.email
        )
    }
}
