/* Standard Library */
use std::{
    io::BufReader,
    error::Error,
};

/* Third Party Libraries */
use serde::{Deserialize, Serialize};
use user_error::UserError;

/* Crate Modules */
use crate::utility;

/* Constants */
const CONFIG_FILENAME: &'static str = "config.json";

const DEFAULT_TITLE: &'static str = "echo";
const DEFAULT_AUTHOR: &'static str = "anonymous";
const DEFAULT_DESCRIPTION: &'static str = "Echo - a microblog generator for \
                                           your web zone";

/* Config structure that reflects the config.json present in individual Echo 
   projects.
*/
#[derive(Serialize, Deserialize)]
pub struct EchoConfig {
    pub title: String,
    pub author: String,
    pub description: String,
}

/* Traits */
impl Default for EchoConfig {
    fn default() -> Self {
        EchoConfig {
            title: String::from(DEFAULT_TITLE),
            author: String::from(DEFAULT_AUTHOR),
            description: String::from(DEFAULT_DESCRIPTION),
        }
    }
}

impl EchoConfig {
    /* Helper functions to keep reading and parsing the config file DRY */
    pub fn get() -> Result<EchoConfig, UserError> {
        /* Open the config.json as a file */
        let file = utility::open_file(CONFIG_FILENAME)?; 

        /* Convert the file into a EchoConfig */
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| {
            UserError::hardcoded(
                "Failed to parse config.json",
                &[e.description()],
                &["Make sure config.json is valid for an Echo project."],
            )
        })
    }
}

