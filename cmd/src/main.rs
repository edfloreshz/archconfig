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
        .about("Confiuration manager for Dotsy.")
        .arg(
            Arg::with_name("daemon")
                .help("Starts Dotsy daemon and shows the output.")
                .short("d")
                .long("daemon"),
        )
        .get_matches()
}
