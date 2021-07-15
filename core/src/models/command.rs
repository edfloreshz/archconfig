use std::env;
use std::fs::File;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::exit;

use clap::ArgMatches;

use crate::cmd::*;
use crate::cmd::daemon::DaemonOptions;
use crate::models::config::{AppOptions, Config, ConfigWriter, UserConfig};

use super::options::FileOptions;
use super::options::OnlineOptions;

pub struct Command {
    subcommand: Subcommand,
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
    pub fn default() -> Command {
        Command { subcommand: Subcommand::None }
    }
    pub fn new(matches: &ArgMatches) -> Option<Command> {
        let sub_matches: &ArgMatches;
        let mut command: Command = Command::default();
        let configuration: AppOptions = AppOptions::default();
        let file: Option<File>;
        let config_file = dirs::data_dir().unwrap_or_default().join("dotsy/config/config.toml");
        let pwd = env::current_dir().unwrap_or(PathBuf::new());
        if matches.is_present("config") {
            let conf = matches.subcommand_matches("config")?;
            if conf.is_present("open") {
                println!("TODO: Open file.");
                //TODO: Open file
                exit(0)
            } else if conf.is_present("reset") {
                println!("TODO: Reset file.");
                //TODO: Reset file.
                exit(0)
            } else if conf.is_present("init") {
                let init = conf.subcommand_matches("init")?;
                let app = if config_file.exists() {
                    let content = toml::from_slice(
                        read_to_string(config_file).unwrap_or_default().as_bytes()
                    ).unwrap_or(AppOptions::default());
                    AppOptions { config: content.config, user: content.user }
                } else {
                    let mut config: Option<Config> = Some(Config::default());
                    if init.is_present("color") { config.as_mut()?.color = true; }
                    AppOptions { config, user: Some(UserConfig::ask()) }
                };
                command.subcommand = Subcommand::Config(app);
            } else {
                command.subcommand = Subcommand::Config(AppOptions::default())
            }
        }
        if matches.is_present("pub") {
            command.subcommand = Subcommand::Publish(OnlineOptions {
                data: UserConfig::new(configuration.clone().user?),
            });
        }
        if matches.is_present("daemon") {
            if matches.subcommand_matches("daemon")?.is_present("show") {
                command.subcommand = Subcommand::Daemon(DaemonOptions { show: true });
            } else {
                command.subcommand = Subcommand::Daemon(DaemonOptions { show: false });
            }
        }
        if matches.is_present("pull") {
            sub_matches = matches.subcommand_matches("pull")?;
            if sub_matches.value_of("user").is_some() && sub_matches.value_of("repo").is_some() {
                command.subcommand = Subcommand::Pull(OnlineOptions {
                    data: UserConfig {
                        provider: configuration.user?.provider,
                        username: sub_matches.value_of("user")?.to_string(),
                        repository: sub_matches.value_of("repo")?.to_string(),
                    },
                });
            } else {
                command.subcommand = Subcommand::Pull(OnlineOptions {
                    data: UserConfig::new(configuration.user?),
                });
            }
        } else if matches.is_present("push") {
            sub_matches = matches.subcommand_matches("push")?;
            if sub_matches.value_of("user").is_some() && sub_matches.value_of("repo").is_some() {
                command.subcommand = Subcommand::Pull(OnlineOptions {
                    data: UserConfig {
                        provider: configuration.user?.provider,
                        username: sub_matches.value_of("user")?.to_string(),
                        repository: sub_matches.value_of("repo")?.to_string(),
                    },
                });
            } else {
                command.subcommand = Subcommand::Push(OnlineOptions {
                    data: UserConfig::new(configuration.user?),
                })
            }
        } else if matches.is_present("add") {
            let file_path = pwd.join(
                matches.subcommand_matches("add")?.value_of("file").unwrap_or_default()
            );
            file = if file_path.exists() { Some(File::open(file_path).unwrap()) } else { None };
            command.subcommand = Subcommand::Add(FileOptions { file });
        } else if matches.is_present("rem") {
            let file_path = pwd.join(
                matches.subcommand_matches("rem")?.value_of("file").unwrap_or_default()
            );
            if file_path.exists() { file = Some(File::open(file_path).unwrap()); } else { file = None; }
            command.subcommand = Subcommand::Remove(FileOptions { file });
        }
        Some(command)
    }
    pub fn execute(&self) -> Result<(), std::io::Error> {
        match &self.subcommand {
            Subcommand::Config(options) => {
                if options.config.is_none() || options.user.is_none() {
                    Ok(println!("Type `dotsy config help` if you need help."))
                } else {
                    config::create()?;
                    options.write_to_config()
                }
            }
            Subcommand::Daemon(options) => {
                if options.show {
                    daemon::show()
                } else {
                    daemon::start()
                }
            }
            Subcommand::Publish(options) => publish::now(options),
            Subcommand::Add(options) => add::now(options),
            Subcommand::Remove(options) => remove::now(options),
            Subcommand::Push(options) => push::now(options),
            Subcommand::Pull(options) => pull::now(options),
            Subcommand::None => Ok(println!("Type `dotsy help` if you need help.")),
        }
    }
}
