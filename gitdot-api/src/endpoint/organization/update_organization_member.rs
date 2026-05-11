use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::OrganizationMemberResource};

pub struct UpdateOrganizationMember;

impl Endpoint for UpdateOrganizationMember {
    const PATH: &'static str = "/organization/{org_name}/member/{member_id}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateOrganizationMemberRequest;
    type Response = UpdateOrganizationMemberResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateOrganizationMemberRequest {
    pub role_description: Option<String>,
}

pub type UpdateOrganizationMemberResponse = OrganizationMemberResource;
