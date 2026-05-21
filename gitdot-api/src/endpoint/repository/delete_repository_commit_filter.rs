use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct DeleteRepositoryCommitFilter;

impl Endpoint for DeleteRepositoryCommitFilter {
    const PATH: &'static str = "/repository/{owner}/{repo}/commit-filters/{filter_id}";
    const METHOD: http::Method = http::Method::DELETE;

    type Request = DeleteRepositoryCommitFilterRequest;
    type Response = DeleteRepositoryCommitFilterResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct DeleteRepositoryCommitFilterRequest {}

pub type DeleteRepositoryCommitFilterResponse = ();
