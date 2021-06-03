use dotsy_core::models::args::{parse_args, Command};

fn main() -> Result<(), std::io::Error> {
    let matches = parse_args();
    let checked = Command::new(&matches);
    checked.unwrap().execute()?;
    Ok(())
}
