use clap::{Arg, ArgMatches, App, SubCommand};

pub fn parse<'a>() -> ArgMatches<'a> {
    App::new("Echo")
        .version("0.0.1")
        .author("Amy Jie <git@xvrqt.com>")
        .about("Write daily journals in the command line and turn them into a static web zone.")
        .subcommand(SubCommand::with_name("init")
            .about("Creates a new Echo project from which you can write daily journal entries and generate a custom web zone.")
            .arg(Arg::with_name("PROJECT_NAME")
                .help("Specify the name of the new Echo project.")))
        .subcommand(SubCommand::with_name("new")
            .about("Run inside an Echo project to create a new Echo entry.")
            .arg(Arg::with_name("PATH")
                .help("Specify the path of the Echo project you wish to create an entry for.")))
        .subcommand(SubCommand::with_name("edit")
            .about("Run inside an Echo project to edit an existing Echo entry.")
            .arg(Arg::with_name("POST_ID")
                .help("Specify the ID of the post you wish to modify. You can view a list of posts by running `echo log`"))
            .arg(Arg::with_name("PATH")
                .help("Specify the path of the Echo project you wish to modify an entry in.")))
        .subcommand(SubCommand::with_name("build")
            .about("Run inside a Echo project to generate a static web blog from your entries.")
            .arg(Arg::with_name("PATH")
                .help("Specify the path of the Echo project you wish to build.")))
        .get_matches()
}

