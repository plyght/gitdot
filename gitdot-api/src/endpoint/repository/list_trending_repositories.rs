use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryResource};

pub struct ListTrendingRepositories;

impl Endpoint for ListTrendingRepositories {
    const PATH: &'static str = "/repository/trending";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListTrendingRepositoriesRequest;
    type Response = ListTrendingRepositoriesResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListTrendingRepositoriesRequest {}

pub type ListTrendingRepositoriesResponse = Vec<RepositoryResource>;
