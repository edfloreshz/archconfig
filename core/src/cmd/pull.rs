use crate::utils::url::GitUrl;

#[derive(Debug)]
pub struct PullOptions {
    pub data: GitUrl,
}

pub fn now(options: &PullOptions) -> Result<(), std::io::Error> {
    println!("Pulling dotfiles...");
    println!("Options: {:?}", options);
    Ok(())
}
