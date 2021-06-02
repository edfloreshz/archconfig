use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    username: String,
    repository: String,
}

pub fn init() {
    let user_info = get_user_data();
    let json = serde_json::to_string(&user_info).unwrap();
    println!("serialized = {}", json);
}

fn get_user_data() -> UserInfo {
    //TODO: Ask for git username and repo.
    UserInfo {
        username: "edfloreshz".into(),
        repository: "data".into(),
    }
}
