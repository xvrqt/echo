use serde::{Deserialize, Serialize};

use crate::post::EchoPost;
use crate::config::EchoConfig;

#[derive(Serialize, Deserialize, Default)]
pub struct EchoContext {
    pub config: EchoConfig,
    pub num_posts: isize,
    pub posts: Vec<EchoPost>
}

