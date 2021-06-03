use crate::models::{config::Config};
use std::{fs};

pub fn start(cmd: &Config) -> Result<(), std::io::Error> {
    let home = dirs::data_dir().unwrap().join("dotsy");
    if !home.exists() {
        fs::create_dir_all(home.join("logs"))?;
        fs::create_dir_all(home.join("config"))?;
        fs::File::create(home.join("logs/daemon.out"))?;
        fs::File::create(home.join("logs/daemon.err"))?;
        fs::File::create(home.join("config/config.toml"))?;
        Ok(())
    } else {
        println!("Configuration already present.");
        Ok(())
    }
}
