use anyhow::{anyhow, Result};

use clap::{App, Arg, ArgMatches, SubCommand};
use dotsy_core::url::{GitUrl, RepoProvider};
use dotsy_daemon::daemon::construct;

fn main() -> Result<()> {
    let args = parse_args();
    if args.is_present("daemon") {
        construct()
    }
    let user = arg_from_subcmd(&args, "pull", "user")?;
    let repo = arg_from_subcmd(&args, "pull", "repo")?;
    let url = GitUrl::new(RepoProvider::GitHub, user, repo);
    println!("{}", url.url());

    Ok(())
}

fn arg_from_subcmd(matches: &ArgMatches, sub: &str, arg: &str) -> Result<String> {
    if let Some(matches) = matches.subcommand_matches(sub) {
        match matches.value_of(arg) {
            Some(e) => Ok(e.into()),
            None => Err(anyhow!("You must provide an argument.")),
        }
    } else {
        Err(anyhow!("You must provide a subcommand."))
    }
}

fn parse_args() -> ArgMatches<'static> {
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
