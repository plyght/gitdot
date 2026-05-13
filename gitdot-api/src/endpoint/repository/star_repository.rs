use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct StarRepository;

impl Endpoint for StarRepository {
    const PATH: &'static str = "/repository/{owner}/{repo}/star";
    const METHOD: http::Method = http::Method::POST;

    type Request = StarRepositoryRequest;
    type Response = StarRepositoryResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct StarRepositoryRequest {}

pub type StarRepositoryResponse = ();
