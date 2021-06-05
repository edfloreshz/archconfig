use crate::models::options::FileOptions;

pub fn now(options: &FileOptions) -> Result<(), std::io::Error> {
    match options.file.as_ref() {
        Some(file) => println!("Removed {:?}\nOptions: {:?}", file, options),
        None => println!("File does not exist."),
    }
    Ok(())
}
