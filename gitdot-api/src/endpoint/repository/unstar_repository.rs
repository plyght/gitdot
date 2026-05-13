use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct UnstarRepository;

impl Endpoint for UnstarRepository {
    const PATH: &'static str = "/repository/{owner}/{repo}/unstar";
    const METHOD: http::Method = http::Method::POST;

    type Request = UnstarRepositoryRequest;
    type Response = UnstarRepositoryResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UnstarRepositoryRequest {}

pub type UnstarRepositoryResponse = ();
