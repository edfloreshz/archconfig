use crate::utils::url::GitUrl;

#[derive(Debug)]
pub struct PublishOptions {
    pub data: GitUrl,
}

pub fn now(options: &PublishOptions) -> Result<(), std::io::Error> {
    println!(
        "Publishing your dotfiles to {}\nOptions: {:?}",
        options.data.url(),
        options
    );
    Ok(())
}
