/* Collection of useful database functions to keep things DRY */

/* Third Party Libraries */
use rusqlite::Connection;
use user_error::UserError;

/* Creates a new post with the content provided */
pub fn new_post(c: Connection, text: &str) -> Result<(), UserError> {
    c.execute(
            "insert into posts (created, edited, text)
             values (
                strftime('%s','now'),
                strftime('%s','now'),
                (?1)
            )",
    &[&text])?;

    Ok(())
}

