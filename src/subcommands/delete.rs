/* Third Party Libraries */
use clap::ArgMatches;
use user_error::UserError;

/* Internal Modules */
use crate::{db, subcommands::build, utility};

/* Edits an existing post in an Echo Project */
pub fn run(args: &ArgMatches) -> Result<isize, UserError> {
    /* Get the project path to build */
    let (_, connection) = utility::get_project(args.value_of("PATH"))?;

    /* Check that the post they want to edit is valid. If no post ID is
       provided, assume most the most recent post.
    */
    let num_posts = db::num_posts(&connection)?;
    if num_posts == 0 {
        return Err(UserError::simple("No posts to edit"));
    }

    /* Parse the post ID, or fall back on the latest post */
    let post_id = args.value_of("POST_ID").map_or(Ok(num_posts - 1), |s| {
        /* Attempts to parse an int from a string */
        s.parse::<isize>()
            .map_err(|e| UserError::hardcoded("Failed to parse post ID", &[&e.to_string()], &[]))
            .and_then(|n| {
                /* Ensure the post ID is within bounds */
                if !(1 <= n && n <= num_posts) {
                    Err(UserError::hardcoded(
                        "Invalid Post ID",
                        &[],
                        &[&format!(
                            "Pick an ID between 1 \
                             and {}",
                            num_posts
                        )],
                    ))
                } else {
                    Ok(n - 1)
                }
            })
    })?;

    println!("{}", post_id);

    /* Commit the post the DB */
    db::delete_post(&connection, post_id)?;

    /* Regenerate the blag */
    build::run(args)?;

    Ok(post_id)
}
