/* Standard Library */
use std::fs;
use std::result::Result;

/* Third Party Libraries */
use clap::ArgMatches;
use user_error::UserError;

/* Internal Modules */
use crate::context::home::EchoHomeContext;
use crate::utility;
use crate::web::templates;

/* Build a static web blog from an existing Echo project */
pub fn run(args: &ArgMatches) -> Result<String, UserError> {
    /* Get the project path to build */
    let (config, connection) = utility::get_project(args.value_of("PATH"))?;

    /* Generate the home page (index.html) */
    let c = EchoHomeContext::new(&config, &connection)?;
    c.write(&"dist/index.html")?;

    Ok(config.title)
}
