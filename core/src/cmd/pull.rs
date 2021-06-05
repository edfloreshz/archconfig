use crate::models::options::OnlineOptions;

pub fn now(options: &OnlineOptions) -> Result<(), std::io::Error> {
    println!("Pulling dotfiles...");
    println!("Options: {:?}", options);
    Ok(())
}
