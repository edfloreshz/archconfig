use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct GitUrl {
    base: RepoProvider,
    user: String,
    repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepoProvider {
    GitHub(String),
    GitLab(String),
    Bitbucket(String),
}

impl GitUrl {
    pub fn new(provider: RepoProvider, user: String, repo: String) -> GitUrl {
        GitUrl {
            base: provider,
            user,
            repo,
        }
    }
    pub fn default(provider: RepoProvider) -> GitUrl {
        GitUrl {
            base: provider,
            user: String::new(), //TODO: pull git user.
            repo: String::new(), //TODO: pull default git repo.
        }
    }
    pub fn url(&self) -> String {
        format!("{:?}/{}/{}", self.base, self.user, self.repo)
    }
}
