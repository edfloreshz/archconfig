use crate::models::config::UserConfig;

#[derive(Debug)]
pub struct PushOptions {
    pub data: UserConfig,
}

pub fn now(options: &PushOptions) -> Result<(), std::io::Error> {
    println!(
        "Pushing dotfiles to {}\nOptions: {:?}",
        options.data.url(),
        options
    );
    Ok(())
}
