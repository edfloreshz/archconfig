use crate::models::config::{AppOptions, Config, ConfigWriter, UserConfig};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;

use crate::cmd::daemon::DaemonOptions;

use super::options::FileOptions;
use super::options::OnlineOptions;

pub struct Command {
    subcmd: Subcommand,
}

enum Subcommand {
    Config(AppOptions),
    Daemon(DaemonOptions),
    Publish(OnlineOptions),
    Add(FileOptions),
    Remove(FileOptions),
    Push(OnlineOptions),
    Pull(OnlineOptions),
    None,
}

impl Command {
    pub fn new(matches: &ArgMatches) -> Option<Command> {
        let subcmd: &ArgMatches;
        let file: Option<File>;
        let pwd = env::current_dir().unwrap_or(PathBuf::new());
        let mut configuration: AppOptions = AppOptions::default();
        let mut cmd = Command {
            subcmd: Subcommand::None,
        };
        let config_file = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::new())
            .join("dotsy/config/config.toml");
        let info = if config_file.exists() {
            let file_content =
                std::fs::read_to_string(config_file).unwrap_or_else(|_| String::new());
            configuration =
                toml::from_str(file_content.as_str()).unwrap_or_else(|_| AppOptions::default());
            Some(configuration.clone().user?)
        } else {
            let provider = matches.subcommand_matches("config")?.value_of("provider")?;
            Some(UserConfig::ask(provider))
        };
        if matches.is_present("config") {
            if matches.subcommand_matches("config")?.is_present("show") {}
            if matches.subcommand_matches("config")?.is_present("color") {
                cmd.subcmd = Subcommand::Config(AppOptions {
                    config: Some(Config { color: true }),
                    user: info,
                });
            } else {
                cmd.subcmd = Subcommand::Config(AppOptions {
                    config: Some(Config { color: false }),
                    user: info,
                });
            }
        }
        if matches.is_present("pub") {
            cmd.subcmd = Subcommand::Publish(OnlineOptions {
                data: UserConfig::new(configuration.clone().user?),
            });
        }
        if matches.is_present("daemon") {
            if matches.subcommand_matches("daemon")?.is_present("show") {
                cmd.subcmd = Subcommand::Daemon(DaemonOptions { show: true });
            } else {
                cmd.subcmd = Subcommand::Daemon(DaemonOptions { show: false });
            }
        }
        if matches.is_present("pull") {
            subcmd = matches.subcommand_matches("pull")?;
            if subcmd.value_of("user").is_some() && subcmd.value_of("repo").is_some() {
                cmd.subcmd = Subcommand::Pull(OnlineOptions {
                    data: UserConfig {
                        provider: configuration.user?.provider,
                        username: subcmd.value_of("user")?.to_string(),
                        repository: subcmd.value_of("repo")?.to_string(),
                    },
                });
            } else {
                cmd.subcmd = Subcommand::Pull(OnlineOptions {
                    data: UserConfig::new(configuration.user?),
                });
            }
        } else if matches.is_present("push") {
            subcmd = matches.subcommand_matches("push")?;
            if subcmd.value_of("user").is_some() && subcmd.value_of("repo").is_some() {
                cmd.subcmd = Subcommand::Pull(OnlineOptions {
                    data: UserConfig {
                        provider: configuration.user?.provider,
                        username: subcmd.value_of("user")?.to_string(),
                        repository: subcmd.value_of("repo")?.to_string(),
                    },
                });
            } else {
                cmd.subcmd = Subcommand::Push(OnlineOptions {
                    data: UserConfig::new(configuration.user?),
                })
            }
        } else if matches.is_present("add") {
            subcmd = matches.subcommand_matches("add")?;
            let file_path = pwd.join(subcmd.value_of("file").unwrap_or_default());
            file = if file_path.exists() {
                Some(File::open(file_path).unwrap())
            } else {
                None
            };
            cmd.subcmd = Subcommand::Add(FileOptions { file });
        } else if matches.is_present("rem") {
            subcmd = matches.subcommand_matches("rem")?;
            let file_path = pwd.join(subcmd.value_of("file").unwrap_or_default());
            if file_path.exists() {
                file = Some(File::open(file_path).unwrap());
            } else {
                file = None;
            }
            cmd.subcmd = Subcommand::Remove(FileOptions { file });
        }
        Some(cmd)
    }
    pub fn execute(&self) -> Result<(), std::io::Error> {
        match &self.subcmd {
            Subcommand::Config(options) => {
                crate::cmd::config::start()?;
                options.write()
            }
            Subcommand::Daemon(options) => {
                if options.show {
                    crate::cmd::daemon::show()
                } else {
                    crate::cmd::daemon::start()
                }
            }
            Subcommand::Publish(options) => crate::cmd::publish::now(options),
            Subcommand::Add(options) => crate::cmd::add::now(options),
            Subcommand::Remove(options) => crate::cmd::remove::now(options),
            Subcommand::Push(options) => crate::cmd::push::now(options),
            Subcommand::Pull(options) => crate::cmd::pull::now(options),
            Subcommand::None => exit(0),
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
