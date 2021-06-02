mod args;
use args::{parse_args, Command};

fn main() {
    let matches = parse_args();
    let checked = Command::new(&matches);
    checked.unwrap().execute();
}
