/* Standard Libraries */
use std::fs;

/* Third Party Libraries */
use serde_json;
use clap::ArgMatches;
use user_error::UserError;

/* Internal Modules */
use crate::{subcommands::build, utility, config};

/* Updates the configuration */
pub fn run(args: &ArgMatches) -> Result<String, UserError> {
    /* Get the project path to build */
    let (mut config, _) = utility::get_project(args.value_of("PATH"))?;

    let field = args.value_of("FIELD").unwrap_or("");
    let value = args.value_of("VALUE").unwrap_or("");
    
    match field {
        "title"  => update_title(&mut config, value),
        "author" => update_author(&mut config, value),
        "description" => update_desc(&mut config, value),
        _ => return Err(UserError::simple("Did not specify a valid field to edit"))
    };

    let config = serde_json::to_string_pretty(&config).expect("Failed conversion of config struct to pretty print JSON string");
    let mut config_path = utility::get_project_root()?;
    config_path.push("config.json");

    /* Write to the file */
    fs::write(config_path, config)?;

    Ok(String::from(field))
}

fn update_title(config: &mut config::EchoConfig, title: &str) {
   config.title = String::from(title); 
}

fn update_author(config: &mut config::EchoConfig, author: &str) {
   config.author= String::from(author); 
}

fn update_desc(config: &mut config::EchoConfig, description: &str) {
   config.description = String::from(description); 
}

