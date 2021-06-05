use crate::models::config::UserConfig;

#[derive(Debug)]
pub struct PublishOptions {
    pub data: UserConfig,
}

pub fn now(options: &PublishOptions) -> Result<(), std::io::Error> {
    println!(
        "Publishing your dotfiles to {}\nOptions: {:?}",
        options.data.url(),
        options
    );
    Ok(())
}
