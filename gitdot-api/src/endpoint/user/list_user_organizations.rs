use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{OrganizationMemberResource, common::Page},
};

pub struct ListUserOrganizations;

impl Endpoint for ListUserOrganizations {
    const PATH: &'static str = "/user/{user_name}/organizations";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserOrganizationsRequest;
    type Response = ListUserOrganizationsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListUserOrganizationsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListUserOrganizationsResponse = Page<OrganizationMemberResource>;
