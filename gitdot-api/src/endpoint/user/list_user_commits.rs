use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{common::Page, user::UserCommitResource},
};

pub struct ListUserCommits;

impl Endpoint for ListUserCommits {
    const PATH: &'static str = "/user/{user_name}/commits";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserCommitsRequest;
    type Response = ListUserCommitsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListUserCommitsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListUserCommitsResponse = Page<UserCommitResource>;
