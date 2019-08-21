use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct EchoPost {
    pub id: i64,
    pub created: i64,
    pub edited: i64,
    pub text: String,
}
