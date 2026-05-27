use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::user::UserEmailResource};

pub struct VerifyUserEmail;

impl Endpoint for VerifyUserEmail {
    const PATH: &'static str = "/auth/email/verify-email";
    const METHOD: http::Method = http::Method::POST;

    type Request = VerifyUserEmailRequest;
    type Response = VerifyUserEmailResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct VerifyUserEmailRequest {
    pub email: String,
    pub code: String,
}

pub type VerifyUserEmailResponse = UserEmailResource;
