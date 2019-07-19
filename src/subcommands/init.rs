/* Standard Library */
use std::fs;
use std::path::Path;
use std::result::Result;

/* Third Party Libraries */
use clap::ArgMatches;
use rusqlite::{Connection, NO_PARAMS};
use user_error::UserError;

/* Internal Modules */
use crate::db;
use crate::config::EchoConfig;

/* Error Messages */
const SUBTLE_HELP:   &str = "Run 'echo init --help' to list all options";
const ERROR_SUMMARY: &str = "Failed to initialize project";

/* Creates a new Echo project */
pub fn run(args: &ArgMatches) -> Result<String, UserError> {
    let project_name = args.value_of("PROJECT_NAME").unwrap_or("echo");
    let directories = vec![format!("{}",            project_name),
                           format!("{}/src",        project_name),
                           format!("{}/src/images", project_name),
                           format!("{}/dist",       project_name)];
    
    /* Check if the project directory exist, create_dir_all is not returning an
       error if it already exists like it should so this check is necessary.
    */
    if Path::new(project_name).exists() {
        let reason = format!("Directory '{}' already exists", project_name);
        return Err(UserError::hardcoded(ERROR_SUMMARY,
                             &[&reason],
                             &["Remove this directory or pick a different name",
                                SUBTLE_HELP]))
    }

    /* Setup the project directory */
    for dir in &directories {
        fs::create_dir_all(dir)?;
    }


    /* Create the config JSON */
    /* Create and open the file */

    /* Set up the config */
    let config = EchoConfig {
        title: String::from(project_name),
        ..Default::default()
    };

    /* Serialize to a pretty print formatted string */
    let config = serde_json::to_string_pretty(&config).expect("Failed conversion of config struct to pretty print JSON string");

    /* Write to the file */
    let config_path = format!("{}/config.json", project_name);
    fs::write(&config_path, config)?;

    /* Setup the SQLite database */
    let db_file_path = format!("{}/echo.db", project_name);
    let conn = Connection::open(db_file_path)?;

    /* Create the posts table */
    conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
                id      integer primary key,
                created integer key,
                edited  integer key,
                text    text not null
            )",
            NO_PARAMS,
    )?;
    
    /* Insert a starter post */
    db::new_post(conn, "it is entirely unfortunate you have come here.")?;

    Ok(String::from(project_name))
}

