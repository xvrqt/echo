/* Standard Library */
use std::fs;
use std::io::BufReader;

/* Third Party Libraries */
use serde::{Deserialize, Serialize};
use user_error::UserError;

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
            author: String::from("anon"),
            description: String::from("Echo - a microblog generator for your web zone")
        }
    }
}

impl EchoConfig {
    /* Helper functions to keep reading and parsing the config file DRY */
    pub fn get() -> Result<EchoConfig, UserError> {
        const ERROR_SUMMARY: &str = "Failed to open config.json";

        /* Open the config.json as a file */
        let file = match fs::File::open("config.json") {
            Ok(file) => file,
            Err(error) => {
                let error = error.to_string();
                return Err(UserError::hardcoded(ERROR_SUMMARY,
                                            &[&error],
                                            &[]));
            }
        };

        /* Convert the file into a EchoConfig */
        let reader = BufReader::new(file);
        let config: EchoConfig = match serde_json::from_reader(reader) {
            Ok(config) => config,
            Err(error) => {
                let error = error.to_string();
                return Err(UserError::hardcoded(ERROR_SUMMARY,
                                            &["Failed to parse config.json", &error],
                                            &["Make sure config.json is valid for an Echo project."]));
            }
        };

        Ok(config)
    }
}

