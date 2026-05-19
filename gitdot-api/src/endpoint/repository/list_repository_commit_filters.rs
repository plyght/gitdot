use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{RepositoryCommitFilterResource, common::Page},
};

pub struct ListRepositoryCommitFilters;

impl Endpoint for ListRepositoryCommitFilters {
    const PATH: &'static str = "/repository/{owner}/{repo}/commit_filters";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListRepositoryCommitFiltersRequest;
    type Response = ListRepositoryCommitFiltersResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListRepositoryCommitFiltersRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListRepositoryCommitFiltersResponse = Page<RepositoryCommitFilterResource>;
