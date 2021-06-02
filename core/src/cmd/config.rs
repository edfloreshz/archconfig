use std::fs;

use serde::{Deserialize, Serialize};
use text_io::read;

#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    username: String,
    repository: String,
}

pub fn init() {
    let user_info = get_user_data();
    let json = serde_json::to_string(&user_info).unwrap();
    let data = fs::File::create("");
    println!("serialized = {}", json);
}

fn create_file_structure() {
    let home = dirs::data_dir().unwrap().join("dotsy");
    if !home.exists() {
        if let Err(e) = fs::create_dir_all(&home.join("logs")) {
            panic!("{} at {:?}", e, home);
        }
        if let Err(e) = fs::create_dir_all(&home.join("config")) {
            panic!("{} at {:?}", e, home);
        }
        let stdout = fs::File::create(home.join("logs/daemon.out")).unwrap();
        let stderr = fs::File::create(home.join("logs/daemon.err")).unwrap();
        let config = fs::File::create(home.join("logs/config.toml")).unwrap();
    } else {
        println!("Configuration already present.");
    }
}

fn get_user_data() -> UserInfo {
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
