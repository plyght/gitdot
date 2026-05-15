use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::RepositoryCommitFilterResource};

pub struct ListRepositoryCommitFilters;

impl Endpoint for ListRepositoryCommitFilters {
    const PATH: &'static str = "/repository/{owner}/{repo}/commit_filters";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListRepositoryCommitFiltersRequest;
    type Response = ListRepositoryCommitFiltersResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListRepositoryCommitFiltersRequest {}

pub type ListRepositoryCommitFiltersResponse = Vec<RepositoryCommitFilterResource>;
