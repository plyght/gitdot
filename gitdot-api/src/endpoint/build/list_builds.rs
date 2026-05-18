use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{BuildResource, common::Page},
};

pub struct ListBuilds;

impl Endpoint for ListBuilds {
    const PATH: &'static str = "/repository/{owner}/{repo}/builds";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListBuildsRequest;
    type Response = ListBuildsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListBuildsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListBuildsResponse = Page<BuildResource>;
