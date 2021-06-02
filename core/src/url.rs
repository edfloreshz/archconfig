pub struct GitUrl {
    base: String,
    user: String,
    repo: String,
}

pub enum RepoProvider {
    GitHub,
    GitLab,
    Bitbucket,
}

impl RepoProvider {
    fn url(&self) -> String {
        match self {
            RepoProvider::GitHub => format!("https://github.com"),
            RepoProvider::GitLab => format!("https://gitlab.org"),
            RepoProvider::Bitbucket => format!("https://bitbucket.org"),
        }
    }
}

impl GitUrl {
    pub fn new(provider: RepoProvider, user: String, repo: String) -> GitUrl {
        GitUrl {
            base: provider.url(),
            user,
            repo,
        }
    }
    pub fn default() -> GitUrl {
        GitUrl {
            base: "".into(),
            user: "".into(),
            repo: "".into(),
        }
    }
    pub fn url(&self) -> String {
        format!("{}/{}/{}", self.base, self.user, self.repo)
    }
}
