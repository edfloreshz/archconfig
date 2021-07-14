use clap::{App, Arg, ArgMatches, SubCommand};

pub fn parse_args() -> ArgMatches<'static> {
    App::new("Dotsy")
        .version("0.1.0")
        .author("Eduardo F. 🥑 <edfloreshz@gmail.com>")
        .about("Dotsy is a configuration manager for UNIX-based systems.")
        .subcommand(
            SubCommand::with_name("config")
                .about("Initializes local configuration.")
                .arg(
                    Arg::with_name("provider")
                        .short("p")
                        .long("provider")
                        .required(true)
                        .index(1)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("color")
                        .short("c")
                        .long("color")
                        .help("Toggle color"),
                )
                .arg(
                    Arg::with_name("open")
                        .short("o")
                        .long("open")
                        .help("Open config file for editing."),
                ),
        )
        .subcommand(
            SubCommand::with_name("pub")
                .about("Publishes local git repository to desired provider."),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a dotfile to tracking.")
                .arg(
                    Arg::with_name("file")
                        .index(1)
                        .multiple(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rem")
                .about("Remove a dotfile from tracking.")
                .arg(
                    Arg::with_name("file")
                        .index(1)
                        .multiple(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("push")
                .about("Push changes to git repo.")
                .arg(
                    Arg::with_name("user")
                        .help("Specify a GitHub user.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("repo")
                        .help("Specify a GitHub repo to push to.")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("pull")
                .about("Pull changes from git repo.")
                .arg(
                    Arg::with_name("user")
                        .help("Specify a GitHub user.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("repo")
                        .help("Specify a GitHub repo to pull from.")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("daemon")
                .about("Starts the daemon and shows the output.")
                .arg(
                    Arg::with_name("show")
                        .help("Show output from daemon.")
                        .short("s")
                        .long("show"),
                ),
        )
        .get_matches()
}
