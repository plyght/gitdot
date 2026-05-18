use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{RunnerResource, common::Page},
};

pub struct ListRunners;

impl Endpoint for ListRunners {
    const PATH: &'static str = "/ci/runner/{owner}";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListRunnersRequest;
    type Response = ListRunnersResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListRunnersRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListRunnersResponse = Page<RunnerResource>;
