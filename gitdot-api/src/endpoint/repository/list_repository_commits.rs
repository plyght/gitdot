use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::default_ref;
use crate::{
    endpoint::Endpoint,
    resource::{common::Page, repository::RepositoryCommitResource},
};

pub struct ListRepositoryCommits;

impl Endpoint for ListRepositoryCommits {
    const PATH: &'static str = "/repository/{owner}/{repo}/commits";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListRepositoryCommitsRequest;
    type Response = ListRepositoryCommitsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListRepositoryCommitsRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListRepositoryCommitsResponse = Page<RepositoryCommitResource>;
