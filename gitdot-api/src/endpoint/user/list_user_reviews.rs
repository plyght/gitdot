use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{common::Page, review::ReviewResource},
};

pub struct ListUserReviews;

impl Endpoint for ListUserReviews {
    const PATH: &'static str = "/user/{user_name}/reviews";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserReviewsRequest;
    type Response = ListUserReviewsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListUserReviewsRequest {
    pub status: Option<String>,
    pub owner: Option<String>,
    pub repo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListUserReviewsResponse = Page<ReviewResource>;
