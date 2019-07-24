/* Standard Library */
use std::fs;
use std::result::Result;

/* Third Party Libraries */
use clap::ArgMatches;
use user_error::UserError;

/* Internal Modules */
use crate::db;
use crate::web::templates;
use crate::utility;
use crate::context::EchoContext;

/* Build a static web blog from an existing Echo project */
pub fn run(args: &ArgMatches) -> Result<String, UserError> {
    /* Get the project path to build */
    let (config, connection) = utility::get_project(args.value_of("PATH"))?;
    
    /* Context for generating the Tera templates */
    let mut context = EchoContext {
        config,
        ..Default::default()
   };

    /* Initialize the context */
    context.posts = db::get_latest(&connection)?;
    context.num_posts = db::num_posts(&connection)?;

    /* Compile Tera templates */
    let index = templates::compile_index(&context)?;

    /* Write index.html to dist/ */
    fs::write("dist/index.html", &index)?;

    Ok(context.config.title)
}


