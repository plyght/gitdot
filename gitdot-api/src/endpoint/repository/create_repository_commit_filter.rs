use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::RepositoryCommitFilterResource};

pub struct CreateRepositoryCommitFilter;

impl Endpoint for CreateRepositoryCommitFilter {
    const PATH: &'static str = "/repository/{owner}/{repo}/commit_filters";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateRepositoryCommitFilterRequest;
    type Response = CreateRepositoryCommitFilterResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateRepositoryCommitFilterRequest {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
}

pub type CreateRepositoryCommitFilterResponse = RepositoryCommitFilterResource;
