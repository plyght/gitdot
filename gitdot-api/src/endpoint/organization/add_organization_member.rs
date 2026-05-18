use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::OrganizationMemberResource};

pub struct AddOrganizationMember;

impl Endpoint for AddOrganizationMember {
    const PATH: &'static str = "/organization/{org_name}/repositories";
    const METHOD: http::Method = http::Method::POST;

    type Request = AddOrganizationMemberRequest;
    type Response = AddOrganizationMemberResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct AddOrganizationMemberRequest {
    pub user_name: String,
    pub role: String,
    pub role_description: Option<String>,
}

pub type AddOrganizationMemberResponse = OrganizationMemberResource;
