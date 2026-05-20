use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::migration::GitHubAppInstallUrlResource};

pub struct GetGitHubAppInstallUrl;

impl Endpoint for GetGitHubAppInstallUrl {
    const PATH: &'static str = "/migration/github/install-url";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetGitHubAppInstallUrlRequest;
    type Response = GetGitHubAppInstallUrlResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetGitHubAppInstallUrlRequest {
    pub action: String,
}

pub type GetGitHubAppInstallUrlResponse = GitHubAppInstallUrlResource;
