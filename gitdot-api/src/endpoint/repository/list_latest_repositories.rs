use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryResource};

pub struct ListLatestRepositories;

impl Endpoint for ListLatestRepositories {
    const PATH: &'static str = "/repository/latest";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListLatestRepositoriesRequest;
    type Response = ListLatestRepositoriesResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListLatestRepositoriesRequest {}

pub type ListLatestRepositoriesResponse = Vec<RepositoryResource>;
