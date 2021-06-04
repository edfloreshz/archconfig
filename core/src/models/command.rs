use crate::models::config::{AppConfig, Config, ConfigWriter, UserConfig};
use crate::utils::url::{GitUrl, RepoProvider};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::exit;

pub struct Command {
    subcmd: Subcommand,
    args: Vec<String>,
}

enum Subcommand {
    Config(AppConfig),
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

        let mut cmd = Command {
            subcmd: Subcommand::None,
            args: vec![],
        };
        if matches.is_present("config") {
            let home = dirs::data_dir()?.join("dotsy");
            let info = if !home.exists() {
                Some(UserConfig::ask())
            } else {
                None
            };
            if matches.subcommand_matches("config")?.is_present("show") {}
            if matches.subcommand_matches("config")?.is_present("color") {
                cmd.subcmd = Subcommand::Config(AppConfig {
                    config: Some(Config { color: true }),
                    user: info,
                });
            } else {
                cmd.subcmd = Subcommand::Config(AppConfig {
                    config: Some(Config { color: false }),
                    user: info,
                });
            }
        }
        if matches.is_present("pub") {
            cmd.subcmd = Subcommand::Publish;
        }
        if matches.is_present("daemon") {
            cmd.subcmd = Subcommand::Daemon;
        }
        if matches.is_present("pull") {
            cmd.subcmd = Subcommand::Pull;
            subcmd = matches.subcommand_matches("pull")?;
            user = subcmd.value_of("user");
            repo = subcmd.value_of("repo");
            if user.is_some() && repo.is_some() {
                cmd.args.push(user.unwrap().to_string());
                cmd.args.push(repo.unwrap().to_string());
            } else {
                return None;
            }
        } else if matches.is_present("push") {
            cmd.subcmd = Subcommand::Push;
            subcmd = matches.subcommand_matches("push")?;
            user = subcmd.value_of("user");
            repo = subcmd.value_of("repo");
            if user.is_some() && repo.is_some() {
                cmd.args.push(user.unwrap().to_string());
                cmd.args.push(repo.unwrap().to_string());
            } else {
                return None;
            }
        } else if matches.is_present("add") {
            cmd.subcmd = Subcommand::Add;
            subcmd = matches.subcommand_matches("add")?;
            file = subcmd.value_of("file");
            if file.is_some() {
                cmd.args.push(file.unwrap().to_string());
            } else {
                return None;
            }
        } else if matches.is_present("rem") {
            cmd.subcmd = Subcommand::Remove;
            subcmd = matches.subcommand_matches("rem")?;
            file = subcmd.value_of("file");
            if file.is_some() {
                cmd.args.push(file.unwrap().to_string());
            } else {
                return None;
            }
        }
        Some(cmd)
    }
    pub fn execute(&self) -> Result<(), std::io::Error> {
        match &self.subcmd {
            Subcommand::Config(config) => {
                crate::cmd::config::start()?;
                config.write()
            }
            Subcommand::Daemon => crate::cmd::daemon::start(),
            // Subcommand::Publish => crate::cmd::publish::now(self.url()),
            // Subcommand::Add => crate::cmd::add::now(),
            // Subcommand::Remove => crate::cmd::remove::now(),
            // Subcommand::Push => crate::cmd::push::now(),
            // Subcommand::Pull => crate::cmd::pull::now(),
            Subcommand::None => exit(0),
            _ => Ok(()),
        }
    }
    pub fn url(&self) -> Option<String> {
        match self.subcmd {
            Subcommand::Push | Subcommand::Pull => {
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
        .author("Eduardo F. 🥑 <edfloreshz@gmail.com>")
        .about("Dotsy is a configuration manager for UNIX-based systems.")
        .subcommand(
            SubCommand::with_name("config")
                .about("Initializes local configuration.")
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
            SubCommand::with_name("daemon").about("Starts the daemon and shows the output."),
        )
        .get_matches()
}
