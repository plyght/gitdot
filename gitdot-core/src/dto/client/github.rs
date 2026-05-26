use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubUser {
    pub login: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubEmail {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubMembership {
    pub state: String,
    pub role: String,
}
