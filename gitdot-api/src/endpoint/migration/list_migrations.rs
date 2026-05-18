use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{common::Page, migration::MigrationResource},
};

pub struct ListMigrations;

impl Endpoint for ListMigrations {
    const PATH: &'static str = "/migrations";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListMigrationsRequest;
    type Response = ListMigrationsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListMigrationsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListMigrationsResponse = Page<MigrationResource>;
