use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::organization::OrganizationResource};

pub struct GetOrganization;

impl Endpoint for GetOrganization {
    const PATH: &'static str = "/organization/{org_name}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetOrganizationRequest;
    type Response = GetOrganizationResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetOrganizationRequest {}

pub type GetOrganizationResponse = OrganizationResource;
