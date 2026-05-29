use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryBlobPairResource};

pub struct GetReviewDiffBlobs;

impl Endpoint for GetReviewDiffBlobs {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/diff/{position}/blobs";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetReviewDiffBlobsRequest;
    type Response = GetReviewDiffBlobsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetReviewDiffBlobsRequest {
    pub revision: Option<i32>,
    pub compare_to: Option<i32>,
}

pub type GetReviewDiffBlobsResponse = Vec<RepositoryBlobPairResource>;
