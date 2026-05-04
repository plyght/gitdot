#[derive(Debug, Clone)]
pub struct VerifySlackBotSignatureRequest {
    pub timestamp: String,
    pub body: Vec<u8>,
    pub signature: String,
}

impl VerifySlackBotSignatureRequest {
    pub fn new(timestamp: String, body: Vec<u8>, signature: String) -> Self {
        Self {
            timestamp,
            body,
            signature,
        }
    }
}
