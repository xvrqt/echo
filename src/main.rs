use user_error::UserError;

mod cli;
mod subcommands;

/* Error Messages */
const ERROR_FAILED_TO_RUN: &str = "Failed to run";
const ERROR_SUBTLY_FAILED_TO_RUN: &str = "You can run 'echo --help' to see a list of all commands and options";

fn main() {


    match cli::parse().subcommand() {
        ("init", Some(m)) => {
            match subcommands::init::run(m) {
                Ok(s) => println!("Created new projext in: {}", s),
                Err(e) => e.print_and_exit()
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
