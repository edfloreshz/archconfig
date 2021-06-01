use clap::{App, Arg, ArgMatches};
use dotsy_daemon::daemon::construct;

fn main() {
    let args = parse_args();
    if args.is_present("daemon") {
        construct()
    }
}

fn parse_args() -> ArgMatches<'static> {
    App::new("Dotsy")
        .version("0.1.0")
        .author("Eduardo F. <edfloreshz@gmail.com>")
        .about("Dotsy is a configuration manager for UNIX-based systems.")
        .arg(
            Arg::with_name("init")
                .help("Creates local git repo to store dotfiles.")
                .short("i")
                .long("init"),
        )
        .arg(
            Arg::with_name("pub")
                .help("Publishes local git repository to desired provider.")
                .short("pub")
                .long("publish"),
        )
        .arg(
            Arg::with_name("add")
                .help("Adds a dotfile to track.")
                .short("a")
                .long("add")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rem")
                .help("Removes a dotfile from tracking.")
                .short("r")
                .long("remove")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("push")
                .help("Push changes to git repo.")
                .long("push")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pull")
                .help("Pull changes from git repo.")
                .long("pull")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("daemon")
                .help("Starts Dotsy daemon and shows the output.")
                .short("d")
                .long("daemon"),
        )
        .get_matches()
}
