use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryBlobPairResource};

pub struct GetRepositoryCommitBlobs;

impl Endpoint for GetRepositoryCommitBlobs {
    const PATH: &'static str = "/repository/{owner}/{repo}/commits/{sha}/blobs";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryCommitBlobsRequest;
    type Response = GetRepositoryCommitBlobsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryCommitBlobsRequest {}

pub type GetRepositoryCommitBlobsResponse = Vec<RepositoryBlobPairResource>;
