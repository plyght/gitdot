use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{common::Page, user::UserRepositoryResource},
};

pub struct ListUserContributedRepositories;

impl Endpoint for ListUserContributedRepositories {
    const PATH: &'static str = "/user/{user_name}/repositories-contributed";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserContributedRepositoriesRequest;
    type Response = ListUserContributedRepositoriesResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListUserContributedRepositoriesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListUserContributedRepositoriesResponse = Page<UserRepositoryResource>;
