use crate::utils::url::GitUrl;

#[derive(Debug)]
pub struct PushOptions {
    pub data: GitUrl,
}

pub fn now(options: &PushOptions) -> Result<(), std::io::Error> {
    println!("Pushing dotfiles to {}\n Options: {:?}", "", options);
    Ok(())
}
