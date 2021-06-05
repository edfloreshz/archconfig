use crate::models::config::UserConfig;

#[derive(Debug)]
pub struct GitUrl {
    base: String,
    user: String,
    repo: String,
}

impl GitUrl {
    pub fn new(data: UserConfig) -> GitUrl {
        GitUrl {
            base: data.provider,
            user: data.username,
            repo: data.repository,
        }
    }
    pub fn url(&self) -> String {
        format!("{}/{}/{}", self.base, self.user, self.repo)
    }
}
