use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{common::Page, question::QuestionResource},
};

pub struct ListQuestions;

impl Endpoint for ListQuestions {
    const PATH: &'static str = "/repository/{owner}/{repo}/questions";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListQuestionsRequest;
    type Response = ListQuestionsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListQuestionsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListQuestionsResponse = Page<QuestionResource>;
