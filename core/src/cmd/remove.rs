use std::fs::File;

#[derive(Debug)]
pub struct RemoveOptions {
    pub file: Option<File>,
}

pub fn now(options: &RemoveOptions) -> Result<(), std::io::Error> {
    match options.file.as_ref() {
        Some(file) => println!("Removed {:?}\nOptions: {:?}", file, options),
        None => println!("File does not exist."),
    }
    Ok(())
}
