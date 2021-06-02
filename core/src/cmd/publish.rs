pub fn now(url: Option<String>) {
    if let Some(url) = url {
        println!("Publishing your dotfiles to {}", url);
    } else {
        println!("No url was provided!",);
    }
}
