use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{OrganizationResource, common::Page},
};

pub struct ListOrganizations;

impl Endpoint for ListOrganizations {
    const PATH: &'static str = "/organizations";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListOrganizationsRequest;
    type Response = ListOrganizationsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListOrganizationsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListOrganizationsResponse = Page<OrganizationResource>;
