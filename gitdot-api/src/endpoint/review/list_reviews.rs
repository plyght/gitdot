use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{common::Page, review::ReviewResource},
};

pub struct ListReviews;

impl Endpoint for ListReviews {
    const PATH: &'static str = "/repository/{owner}/{repo}/reviews";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListReviewsRequest;
    type Response = ListReviewsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListReviewsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListReviewsResponse = Page<ReviewResource>;
