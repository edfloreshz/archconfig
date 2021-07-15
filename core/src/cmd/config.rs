use std::{fs, path::PathBuf};

pub fn create() -> Result<(), std::io::Error> {
    let data_dir = dirs::data_dir().unwrap_or(PathBuf::new()).join("dotsy");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir.join("logs"))?;
        println!("Logs directory located at: {}", data_dir.join("logs").display());
        fs::create_dir_all(data_dir.join("config"))?;
        println!("Config directory located at: {}", data_dir.join("config").display());
        fs::File::create(data_dir.join("logs/daemon.out"))?;
        fs::File::create(data_dir.join("logs/daemon.err"))?;
        fs::File::create(data_dir.join("config/config.toml"))?;
    } else {
        println!("Configuration already present.");
    }
    Ok(())
}

pub fn open() -> Result<(), std::io::Error> {
    let data_dir = dirs::data_dir().unwrap_or(PathBuf::new()).join("dotsy");
    if data_dir.exists() {
        println!("Opening...");
        //TODO: Open config file for editing.
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Configuration file not found"))
    }
}
