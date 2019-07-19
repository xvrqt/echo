use serde::{Deserialize, Serialize};

use crate::post::EchoPost;
use crate::config::EchoConfig;

#[derive(Serialize, Deserialize, Default)]
pub struct EchoContext {
    pub config: EchoConfig,
    pub num_posts: u64,
    pub posts: Vec<EchoPost>
}

