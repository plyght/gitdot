use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct AddUserEmail;

impl Endpoint for AddUserEmail {
    const PATH: &'static str = "/auth/account/add-email";
    const METHOD: http::Method = http::Method::POST;

    type Request = AddUserEmailRequest;
    type Response = ();
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct AddUserEmailRequest {
    pub email: String,
}
