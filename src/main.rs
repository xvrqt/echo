use user_error::UserError;

mod db;
mod cli;
mod web;
mod post;
mod config;
mod context;
mod utility;
mod subcommands;

fn main() {

    /* Error Messages */
    const ERROR_FAILED_TO_RUN: &str = "Failed to run";
    const ERROR_SUBTLY_FAILED_TO_RUN: &str = "You can run 'echo --help' to see a list of all commands and options";

    match cli::parse().subcommand() {
        ("init", Some(m)) => {
            match subcommands::init::run(m) {
                Ok(s) => println!("Created new project in: {}", s),
                Err(mut e) => {
                    e.update_and_push_summary("Failed to initialize project");
                    e.print_and_exit()
                }
            }
        },
        ("new", Some(m)) => {
            match subcommands::new::run(m) {
                Ok(s) => println!("Created new Echo entry: {}", &s[..15]),
                Err(mut e) => {
                    e.update_and_push_summary("Failed to create new entry");
                    e.print_and_exit()
                }
            }
        },
        ("edit", Some(m)) => {
            match subcommands::edit::run(m) {
                Ok(s) => println!("Updated Echo entry #{}", s + 1),
                Err(mut e) => {
                    e.update_and_push_summary("Failed to modify entry");
                    e.print_and_exit()
                }
            }
        },
        ("delete", Some(m)) => {
            match subcommands::delete::run(m) {
                Ok(s) => println!("Removed Echo entry #{}", s + 1),
                Err(mut e) => {
                    e.update_and_push_summary("Failed to delete entry");
                    e.print_and_exit()
                }
            }
        },
        ("build", Some(m)) => {
            match subcommands::build::run(m) {
                Ok(s) => println!("Built project: {}", s),
                Err(mut e) => {
                    e.update_and_push_summary("Failed to build Echo project");
                    e.print_and_exit()
                }
            }
        },
        (_, None) => {
            UserError::hardcoded(ERROR_FAILED_TO_RUN,
                                 &["No command provided"],
                                 &[ERROR_SUBTLY_FAILED_TO_RUN])
                                .print_and_exit();
        },
        _ => {
            UserError::hardcoded(ERROR_FAILED_TO_RUN,
                                 &["Unknown command provided"],
                                 &[ERROR_SUBTLY_FAILED_TO_RUN])
                                 .print_and_exit();
        }
    }
}
