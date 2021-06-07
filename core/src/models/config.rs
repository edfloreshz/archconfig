use serde::{Deserialize, Serialize};
use std::{io::Write, path::PathBuf};
use text_io::read;

pub trait ConfigWriter {
    fn write(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppOptions {
    pub config: Option<Config>,
    pub user: Option<UserConfig>,
}

impl AppOptions {
    pub fn default() -> AppOptions {
        AppOptions {
            config: None,
            user: None,
        }
    }
}

impl ConfigWriter for AppOptions {
    fn write(&self) -> Result<(), std::io::Error> {
        println!("{:?}", self);
        let home = dirs::data_dir().unwrap_or(PathBuf::new()).join("dotsy");
        if self.user.is_some() {
            let mut config_file = std::fs::File::create(home.join("config/config.toml"))?;
            config_file
                .write_all(toml::to_string(self).unwrap_or(String::new()).as_bytes())
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
    pub provider: String,
}

impl UserConfig {
    pub fn ask(provider: &str) -> UserConfig {
        UserConfig {
            username: {
                println!("Type your Git username: ");
                read!()
            },
            repository: {
                println!("Type your Git repository: ");
                read!()
            },
            provider: match provider {
                "GitHub" | "github" | "Github" => "https://github.com".into(),
                "GitLab" | "gitlab" | "Gitlab" => "https://gitlab.com".into(),
                "Bitbucket" | "bitbucket" => "https://bitbucket.com".into(),
                _ => panic!("Enter a valid provider."),
            },
        }
    }
    pub fn new(data: UserConfig) -> UserConfig {
        UserConfig {
            provider: data.provider,
            username: data.username,
            repository: data.repository,
        }
    }
    pub fn url(&self) -> String {
        format!("{}/{}/{}", self.provider, self.username, self.repository)
    }
}
