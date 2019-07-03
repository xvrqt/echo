use clap::{Arg, App, SubCommand};
use user_error::UserError;

mod subcommands;

fn main() {
    let matches = App::new("Echo")
                          .version("0.0.1")
                          .author("Amy Jie <git@xvrqt.com>")
                          .about("Write daily journals in the command line and turn them into a static web zone.")
                          .subcommand(SubCommand::with_name("init")
                                      .about("Creates a new Echo project from which you can write daily journal entries and generate a custom web zone.")
                                      .arg(Arg::with_name("PROJECT_NAME")
                                          .help("Specify the name of the new Echo project.")))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        let project_name = matches.value_of("PROJECT_NAME").unwrap_or("echo");
        println!("Project Name: {}", project_name);
        subcommands::init::run();
    } else {
        let ue = UserError::simple("Nya");
        ue.print_and_exit();
    }
}
