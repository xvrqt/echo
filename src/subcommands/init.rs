/* Standard Library */
use std::env;
use std::fs;
use std::path::Path;
use std::result::Result;

/* Third Party Libraries */
use clap::ArgMatches;
use rusqlite::NO_PARAMS;
use user_error::UserError;

/* Internal Modules */
use crate::config::EchoConfig;
use crate::db;

/* Creates a new Echo project */
pub fn run(args: &ArgMatches) -> Result<String, UserError> {
    let project_name = args.value_of("PROJECT_NAME").unwrap_or("echo");
    let project_path = Path::new(project_name);

    /* Check if the project directory exist, create_dir_all is not returning an
       error if it already exists like it should so this check is necessary.
    */
    if project_path.exists() {
        let summary = format!("Directory '{}' already exists", project_name);
        return Err(UserError::hardcoded(
            &summary,
            &[],
            &["Remove this directory or pick a different name"],
        ));
    } else {
        fs::create_dir(project_name)?;
        env::set_current_dir(project_path)?;
    }

    /* Setup the project directory */
    let directories = vec!["src", "src/images", "dist"];
    for dir in &directories {
        fs::create_dir_all(dir)?;
    }

    /* Set up the config */
    let config = EchoConfig {
        title: String::from(project_name),
        ..Default::default()
    };

    /* Serialize to a pretty print formatted string */
    let config = serde_json::to_string_pretty(&config)
        .expect("Failed conversion of config struct to pretty print JSON string");

    /* Write to the file */
    fs::write("config.json", config)?;

    /* Setup the SQLite database */
    let conn = db::connect()?;

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
