use std::process::exit;

use clap::{App, Arg, ArgMatches, SubCommand};
use dotsy_core::utils::url::{GitUrl, RepoProvider};

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
    pub fn new(matches: &ArgMatches) -> Option<Command> {
        let subcmd: &ArgMatches;
        let user: Option<&str>;
        let repo: Option<&str>;
        let file: Option<&str>;
        let mut command = Command {
            subcommand: Subcomand::None,
            args: vec![],
        };
        if matches.is_present("init") {
            command.subcommand = Subcomand::Init;
        }
        if matches.is_present("pub") {
            command.subcommand = Subcomand::Publish;
        }
        if matches.is_present("daemon") {
            command.subcommand = Subcomand::Daemon;
        }
        if matches.is_present("pull") {
            command.subcommand = Subcomand::Pull;
            subcmd = matches.subcommand_matches("pull")?;
            user = subcmd.value_of("user");
            repo = subcmd.value_of("repo");
            if user.is_some() && repo.is_some() {
                command.args.push(user.unwrap().to_string());
                command.args.push(repo.unwrap().to_string());
            } else {
                return None;
            }
        } else if matches.is_present("push") {
            command.subcommand = Subcomand::Push;
            subcmd = matches.subcommand_matches("push")?;
            user = subcmd.value_of("user");
            repo = subcmd.value_of("repo");
            if user.is_some() && repo.is_some() {
                command.args.push(user.unwrap().to_string());
                command.args.push(repo.unwrap().to_string());
            } else {
                return None;
            }
        } else if matches.is_present("add") {
            command.subcommand = Subcomand::Add;
            subcmd = matches.subcommand_matches("add")?;
            file = subcmd.value_of("file");
            if file.is_some() {
                command.args.push(file.unwrap().to_string());
            } else {
                return None;
            }
        } else if matches.is_present("rem") {
            command.subcommand = Subcomand::Remove;
            subcmd = matches.subcommand_matches("rem")?;
            file = subcmd.value_of("file");
            if file.is_some() {
                command.args.push(file.unwrap().to_string());
            } else {
                return None;
            }
        }
        Some(command)
    }
    pub fn execute(&self) {
        match self.subcommand {
            Subcomand::Init => dotsy_core::cmd::config::init(),
            Subcomand::Daemon => dotsy_daemon::daemon::construct(),
            Subcomand::Publish => dotsy_core::cmd::publish::now(self.url()),
            Subcomand::Add => dotsy_core::cmd::add::now(),
            Subcomand::Remove => dotsy_core::cmd::remove::now(),
            Subcomand::Push => dotsy_core::cmd::push::now(),
            Subcomand::Pull => dotsy_core::cmd::pull::now(),
            Subcomand::None => exit(0),
        }
    }
    pub fn url(&self) -> Option<String> {
        match self.subcommand {
            Subcomand::Push | Subcomand::Pull => {
                if self.args.len() == 2 {
                    Some(
                        GitUrl::new(
                            RepoProvider::GitHub,
                            self.args[0].clone(),
                            self.args[1].clone(),
                        )
                        .url(),
                    )
                } else {
                    Some(GitUrl::default(RepoProvider::GitHub).url())
                }
            }
            _ => None,
        }
    }
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
