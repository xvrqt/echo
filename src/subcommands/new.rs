/* Third Party Libraries */
use clap::ArgMatches;
use user_error::UserError;

/* Internal Modules */
use crate::{db, subcommands::build, utility};

/* Adds a new post to an Echo Project */
pub fn run(args: &ArgMatches) -> Result<String, UserError> {
    /* Get the project path to build */
    let (_, connection) = utility::get_project(args.value_of("PATH"))?;

    /* Open a text editor for the user to write a post in */
    let post = scrawl::new()?;

    /* Commit the post the DB */
    db::new_post(connection, &post)?;

    /* Regenerate the blag */
    build::run(args)?;

    let summary = String::from(&post);
    let summary = summary.chars().take(15).collect();
    Ok(summary)
}
