use dotsy_core::models::command::{parse_args, Command};

fn main() -> Result<(), std::io::Error> {
    let matches = parse_args();
    let checked = Command::new(&matches);
    match checked {
        Some(cmd) => cmd.execute()?,
        None => eprintln!("Hmm, that can't be right."),
    }
    Ok(())
}
