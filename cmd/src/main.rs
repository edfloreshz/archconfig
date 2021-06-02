mod args;
use anyhow::Result;
use args::{check_matches, parse_args};
fn main() -> Result<()> {
    let matches = parse_args();
    let checked = check_matches(&matches);

    if checked.is_some() {
        println!("{}", checked.unwrap().url());
    }
    Ok(())
}
