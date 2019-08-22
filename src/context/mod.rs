/* Third Party Libraries */
use serde::{Deserialize, Serialize};

/* Crate Modules */
use crate::config::EchoConfig;
use crate::post::EchoPost;

#[derive(Serialize, Deserialize, Default)]
pub struct EchoContext {
    pub config: EchoConfig,
    pub num_posts: isize,
    pub posts: Vec<Vec<EchoPost>>,
}

