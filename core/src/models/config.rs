use serde::{Deserialize, Serialize};
use std::io::Write;
use text_io::read;

pub trait ConfigWriter {
    fn write(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub config: Option<Config>,
    pub user: Option<UserConfig>,
}

impl ConfigWriter for AppConfig {
    fn write(&self) -> Result<(), std::io::Error> {
        let home = dirs::data_dir().unwrap().join("dotsy");
        if self.user.is_some() {
            let mut config_file = std::fs::File::create(home.join("config/config.toml"))?;
            config_file
                .write_all(toml::to_string(self).unwrap().as_bytes())
                .expect("Unable to write data.");
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub color: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub username: String,
    pub repository: String,
}

impl UserConfig {
    pub fn ask() -> UserConfig {
        UserConfig {
            username: {
                println!("Type your Git username: ");
                read!()
            },
            repository: {
                println!("Type your Git repository: ");
                read!()
            },
        }
    }
    pub fn default() -> UserConfig {
        UserConfig {
            username: String::new(),
            repository: String::new(),
        }
    }
}
