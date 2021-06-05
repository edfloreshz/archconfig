use crate::models::options::OnlineOptions;

pub fn now(options: &OnlineOptions) -> Result<(), std::io::Error> {
    println!(
        "Pushing dotfiles to {}\nOptions: {:?}",
        options.data.url(),
        options
    );
    Ok(())
}
