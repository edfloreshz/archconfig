use dotsy_core::cmd::args::parse_args;
use dotsy_core::models::command::Command;

fn main() -> Result<(), std::io::Error> {
    match Command::new(&parse_args()) {
        Some(cmd) => cmd.execute()?,
        None => eprintln!("Type `dotsy help` if you need help."),
    }
    Ok(())
}
