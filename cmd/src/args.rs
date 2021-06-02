use clap::{App, Arg, ArgMatches, SubCommand};
use dotsy_core::url::{GitUrl, RepoProvider};
use dotsy_daemon::daemon::construct;

pub struct Command {
    subcommand: Subcomand,
    args: Vec<String>,
}

enum Subcomand {
    Init,
    Daemon,
    Publish,
    Add,
    Remove,
    Push,
    Pull,
    None,
}

impl Command {
    pub fn url(&self) -> String {
        match self.subcommand {
            Subcomand::Push | Subcomand::Pull => {
                if self.args.len() == 2 {
                    GitUrl::new(
                        RepoProvider::GitHub,
                        self.args[0].clone(),
                        self.args[1].clone(),
                    )
                    .url()
                } else {
                    GitUrl::default(RepoProvider::GitHub).url()
                }
            }
            _ => GitUrl::default(RepoProvider::GitHub).url(),
        }
    }
}

pub fn check_matches(matches: &ArgMatches) -> Option<Command> {
    let mut command = Command {
        subcommand: Subcomand::None,
        args: vec![],
    };
    let mut sub: &ArgMatches;
    let mut user: Option<&str>;
    let mut repo: Option<&str>;
    let mut file: Option<&str>;
    if matches.is_present("init") {
        command.subcommand = Subcomand::Init;
        dotsy_core::config::init();
    }
    if matches.is_present("pub") {
        command.subcommand = Subcomand::Publish;
    }
    if matches.is_present("daemon") {
        command.subcommand = Subcomand::Daemon;
        construct()
    }
    if matches.is_present("pull") {
        command.subcommand = Subcomand::Pull;
        sub = matches.subcommand_matches("pull")?;
        user = sub.value_of("user");
        repo = sub.value_of("repo");
        if user.is_some() && repo.is_some() {
            command.args.push(user.unwrap().to_string());
            command.args.push(repo.unwrap().to_string());
        }
    }
    if matches.is_present("push") {
        command.subcommand = Subcomand::Push;
        sub = matches.subcommand_matches("push")?;
        user = sub.value_of("user");
        repo = sub.value_of("repo");
        if user.is_some() && repo.is_some() {
            command.args.push(user.unwrap().to_string());
            command.args.push(repo.unwrap().to_string());
        }
    }
    if matches.is_present("add") {
        command.subcommand = Subcomand::Add;
        sub = matches.subcommand_matches("add")?;
        file = sub.value_of("file");
        if file.is_some() {
            command.args.push(file.unwrap().to_string())
        }
    }
    if matches.is_present("rem") {
        command.subcommand = Subcomand::Remove;
        let sub = matches.subcommand_matches("rem")?;
        file = sub.value_of("file");
        if file.is_some() {
            command.args.push(file.unwrap().to_string())
        }
    }
    Some(command)
}

pub fn parse_args() -> ArgMatches<'static> {
    App::new("Dotsy")
        .version("0.1.0")
        .author("Eduardo F. <edfloreshz@gmail.com>")
        .about("Dotsy is a configuration manager for UNIX-based systems.")
        .subcommand(
            SubCommand::with_name("init").about("Creates local git repo to store dotfiles."),
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
            SubCommand::with_name("daemon").about("Starts the daemon and shows the output."),
        )
        .get_matches()
}

pub fn get_sub_arg(matches: &ArgMatches, sub: &str, arg: &str) -> Option<String> {
    if let Some(matches) = matches.subcommand_matches(sub) {
        match matches.value_of(arg) {
            Some(e) => Some(e.into()),
            None => None,
        }
    } else {
        None
    }
}
