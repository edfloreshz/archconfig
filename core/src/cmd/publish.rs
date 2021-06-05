use crate::models::options::OnlineOptions;

pub fn now(options: &OnlineOptions) -> Result<(), std::io::Error> {
    println!(
        "Publishing your dotfiles to {}\nOptions: {:?}",
        options.data.url(),
        options
    );
    Ok(())
}
