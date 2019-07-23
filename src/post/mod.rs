use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EchoPost {
    pub created: i64,
    pub edited: i64,
    pub text: String
}

