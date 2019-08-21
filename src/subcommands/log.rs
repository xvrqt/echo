/* Standard Library */
use std::fs;
use std::result::Result;

/* Third Party Libraries */
use clap::ArgMatches;
use user_error::UserError;

/* Internal Modules */
use crate::context::EchoContext;
use crate::db;
use crate::utility;
use crate::web::templates;

/* Lists summarized versions of each post to less */
pub fn run(args: &ArgMatches) -> Result<(), UserError> {
    /* Get the project path to build */
    let (config, connection) = utility::get_project(args.value_of("PATH"))?;

    /* Initialize the context */
    let posts = db::get_latest(&connection)?;

    Ok(())

}

