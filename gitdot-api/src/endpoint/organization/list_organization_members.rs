use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{OrganizationMemberResource, common::Page},
};

pub struct ListOrganizationMembers;

impl Endpoint for ListOrganizationMembers {
    const PATH: &'static str = "/organization/{org_name}/members";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListOrganizationMembersRequest;
    type Response = ListOrganizationMembersResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListOrganizationMembersRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListOrganizationMembersResponse = Page<OrganizationMemberResource>;
