use std::fs;
use std::path::Path;
use std::result::Result;

use clap::{Arg, ArgMatches, App, SubCommand};
use rusqlite::{Connection, Result as SQLResult, NO_PARAMS};

use user_error::UserError;

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

    /* Setup the SQLite database */
    let db_file_path = format!("{}/echo.db", project_name);
    let conn = Connection::open(db_file_path)?;

    conn.execute(
            "CREATE TABLE IF NOT EXISTS echo (
                id      integer primary key,
                created integer key,
                edited  integer key,
                post    text not null
            )",
            NO_PARAMS,
    )?;

    Ok(String::from(project_name))
}

