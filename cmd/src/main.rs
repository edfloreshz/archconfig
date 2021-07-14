use dotsy_core::cmd::args::parse_args;
use dotsy_core::models::command::Command;

fn main() -> Result<(), std::io::Error> {
    let matches = parse_args();
    let checked = Command::new(&matches);
    match checked {
        Some(cmd) => cmd.execute()?,
        None => eprintln!("Hmm, that can't be right."),
    }
    Ok(())
}
