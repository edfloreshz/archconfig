use crate::models::config::UserConfig;

#[derive(Debug)]
pub struct PullOptions {
    pub data: UserConfig,
}

pub fn now(options: &PullOptions) -> Result<(), std::io::Error> {
    println!("Pulling dotfiles...");
    println!("Options: {:?}", options);
    Ok(())
}
