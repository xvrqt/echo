use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct EchoPost {
    pub created: u64,
    pub edited: u64,
    pub text: String
}

