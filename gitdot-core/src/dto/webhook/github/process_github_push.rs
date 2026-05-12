use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessGithubPushRequest {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub before: String,
    pub after: String,
    pub repository: GithubRepository,
    pub pusher: GithubPusher,
    pub installation: GithubInstallation,
    pub commits: Vec<GithubPushCommit>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubRepository {
    pub id: i64,
    pub name: String,
    pub owner: GithubRepositoryOwner,
    pub default_branch: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubRepositoryOwner {
    pub login: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubPusher {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubInstallation {
    pub id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubPushCommit {
    pub id: String,
    pub message: String,
}
