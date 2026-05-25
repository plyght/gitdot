#[derive(Debug, Clone)]
pub struct VerifyGithubSignatureRequest {
    pub body: Vec<u8>,
    pub signature: String,
}

impl VerifyGithubSignatureRequest {
    pub fn new(body: Vec<u8>, signature: String) -> Self {
        Self { body, signature }
    }
}
