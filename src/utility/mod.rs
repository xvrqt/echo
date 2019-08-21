/* Standard Library */
use std::env;
use std::path::Path;
use std::path::PathBuf;

/* Third Party Libraries */
use rusqlite::Connection;
use user_error::UserError;

/* Internal Modules */
use crate::config::EchoConfig;
use crate::db;

/* Searches for the config.json and echo.db on the way from the provided path
   to root. If found, it parses the config into an EchoConfig and opens a
   connection to the DB.

   PARAMS: Takes an optional PathBuf, otherwise assumes

   RETURNS: Result tuple containing an (EchoConfig, Connection) or a
            Err(UserError).

   SIDE EFFECTS: Changes the CWD to the Echo project root.
*/
pub fn get_project(path: Option<&str>) -> Result<(EchoConfig, Connection), UserError> {
    /* Use the CWD as the path if None was provided */
    let path = match path {
        Some(path) => Path::new(path).to_owned(),
        None => get_project_root()?,
    };

    /* Check that the path is a directory, and change into it */
    if !path.is_dir() {
        let path = path.to_str().unwrap_or("<unknown>");
        let reason = format!("{} is not a directory", path);
        return Err(UserError::hardcoded(
            "Invalid path to Echo project",
            &[&reason],
            &[],
        ));
    }
    env::set_current_dir(path)?;

    /* Grab the config and database connection */
    let config = EchoConfig::get()?;
    let connection = db::connect()?;
    Ok((config, connection))
}

/* Returns the PathBuf to the root of an Echo project starting from the cwd and
 * tracing up to root. Returns a UserError if no Echo project is found.
 * e.g. if a user runs 'echo build' in /home/user/echo/dist it will return a
 * PathBuf containing /home/user/echo
*/
pub fn get_project_root() -> Result<PathBuf, UserError> {
    let path = Path::new(".").canonicalize()?;

    /* Check for a config.json and an echo.db in the directory */
    for path in path.ancestors() {
        if is_echo_root(path) {
            return Ok(path.to_owned());
        }
    }

    Err(UserError::hardcoded(
        "Not inside an Echo project",
        &["Did not detect Echo project in path to root"],
        &[
            "An Echo project directory has both a config.json and echo.db file.",
            "You can create a new Echo project by running `echo init <name>`",
            "Run `echo -h` for more details and options.",
        ],
    ))
}

/* Helper function for get_project_root() - returns true if in the root of an
 * Echo project.
*/
fn is_echo_root(path: &Path) -> bool {
    let db_path = path.join("echo.db");
    let config_path = path.join("config.json");

    db_path.exists() && config_path.exists()
}
