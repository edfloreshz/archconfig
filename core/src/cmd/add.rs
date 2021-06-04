use std::fs::File;

#[derive(Debug)]
pub struct AddOptions {
    pub file: Option<File>,
}

pub fn now(options: &AddOptions) -> Result<(), std::io::Error> {
    match options.file.as_ref() {
        Some(file) => println!("Added {:?}\nOptions: {:?}", file, options),
        None => println!("File does not exist."),
    }
    Ok(())
}
