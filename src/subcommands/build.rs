/* Standard Library */
use std::fs;
use std::path::Path;
use std::result::Result;

/* Third Party Libraries */
use clap::ArgMatches;
use rusqlite::{Connection, NO_PARAMS};
use user_error::UserError;

#[macro_use]
use lazy_static_include;

/* Internal Modules */
use crate::db;

/* Error Messages */
const ERROR_SUMMARY: &str = "Failed to build project";

/* Import the files in the template as strings */
/* lazy_static_include_str!(HEAD,   "template/head.html"); */
/* lazy_static_include_str!(POST,   "template/post.html"); */
/* lazy_static_include_str!(INDEX,  "template/index.html"); */
/* lazy_static_include_str!(HEADER, "template/header.html"); */
/* lazy_static_include_str!(FOOTER, "template/footer.html"); */

/* Build a static web blog from an existing Echo project */
pub fn run(args: &ArgMatches) -> Result<String, UserError> {
    let path = args.value_of("PATH").unwrap_or(".");
    let project = Path::new(path);
    if !project.is_dir() {
        let reason = format!("{} is not a directory", path);
        return Err(UserError::hardcoded(ERROR_SUMMARY,
                                    &[&reason],
                                    &[]));
    }

    

    Ok(String::from("u gay"))
}

