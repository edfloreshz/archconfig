use clap::ArgMatches;
use std::env;
use std::fs::File;
use std::path::PathBuf;

use crate::cmd::*;
use crate::cmd::daemon::DaemonOptions;
use crate::models::config::{AppOptions, Config, ConfigWriter, UserConfig};

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
            .unwrap_or(PathBuf::new())
            .join("dotsy/config/config.toml");
        let info = if config_file.exists() {
            let file_content = std::fs::read_to_string(config_file).unwrap_or(String::new());
            configuration = toml::from_str(file_content.as_str()).unwrap_or(AppOptions::default());
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
                config::start()?;
                options.write()
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
