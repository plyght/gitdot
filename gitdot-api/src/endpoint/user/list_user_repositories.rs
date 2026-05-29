use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{common::Page, user::UserRepositoryResource},
};

pub struct ListUserRepositories;

impl Endpoint for ListUserRepositories {
    const PATH: &'static str = "/user/{user_name}/repositories";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserRepositoriesRequest;
    type Response = ListUserRepositoriesResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListUserRepositoriesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListUserRepositoriesResponse = Page<UserRepositoryResource>;
