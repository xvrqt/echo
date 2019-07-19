use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EchoConfig {
    pub title: String,
    pub author: String,
    pub description: String
}

impl Default for EchoConfig {
    fn default() -> Self {
        EchoConfig {
            title: String::from("echo"),
            author: String::from("anonymous"),
            description: String::from("Echo - a microblog generator for your web zone")
        }
    }
}

