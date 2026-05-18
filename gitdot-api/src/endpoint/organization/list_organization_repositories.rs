use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{RepositoryResource, common::Page},
};

pub struct ListOrganizationRepositories;

impl Endpoint for ListOrganizationRepositories {
    const PATH: &'static str = "/organization/{org_name}/repositories";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListOrganizationRepositoriesRequest;
    type Response = ListOrganizationRepositoriesResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListOrganizationRepositoriesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListOrganizationRepositoriesResponse = Page<RepositoryResource>;
