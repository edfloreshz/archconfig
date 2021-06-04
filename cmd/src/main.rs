use dotsy_core::models::command::{parse_args, Command};

fn main() -> Result<(), std::io::Error> {
    let matches = parse_args();
    let checked = Command::new(&matches);
    checked.unwrap().execute()?;
    Ok(())
}
