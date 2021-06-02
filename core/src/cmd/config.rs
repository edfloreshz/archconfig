use serde::{Deserialize, Serialize};
use std::{fs, io::Write};
use text_io::read;

pub fn init() -> Result<(), std::io::Error> {
    let user_info = UserInfo::default();
    let toml = toml::to_string(&user_info).unwrap();
    if let Err(e) = create_file_structure(toml) {
        eprintln!("{}", e)
    }
    Ok(())
}

fn create_file_structure(data: String) -> Result<(), std::io::Error> {
    let home = dirs::data_dir().unwrap().join("dotsy");
    if !home.exists() {
        fs::create_dir_all(home.join("logs"))?;
        fs::create_dir_all(home.join("config"))?;
        fs::File::create(home.join("logs/daemon.out"))?;
        fs::File::create(home.join("logs/daemon.err"))?;
        let mut config = fs::File::create(home.join("config/config.toml"))?;
        config
            .write_all("[config]".as_bytes())
            .expect("Unable to write data.");
        let mut user = fs::File::create(home.join("config/user.toml"))?;
        user.write_all(data.as_bytes())
            .expect("Unable to write data.");
        Ok(())
    } else {
        println!("Configuration already present.");
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    username: String,
    repository: String,
}

impl UserInfo {
    fn default() -> UserInfo {
        UserInfo {
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
}
