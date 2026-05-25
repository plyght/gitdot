use serde::{Deserialize, Serialize};

use super::default_visibility;
use crate::{endpoint::Endpoint, resource::RepositoryResource};

pub struct CreateRepository;

impl Endpoint for CreateRepository {
    const PATH: &'static str = "/repository/{owner}/{repo}";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateRepositoryRequest;
    type Response = CreateRepositoryResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateRepositoryRequest {
    pub owner_type: String,

    #[serde(default = "default_visibility")]
    pub visibility: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub init_readme: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gitignore_template: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_template: Option<String>,
}

pub type CreateRepositoryResponse = RepositoryResource;
