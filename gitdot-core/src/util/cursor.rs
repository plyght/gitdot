use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::InputError;

pub fn encode<T: Serialize>(value: &T) -> String {
    let json = serde_json::to_vec(value).expect("cursor value must serialize");
    URL_SAFE_NO_PAD.encode(json)
}

pub fn decode<T: DeserializeOwned>(s: &str) -> Result<T, InputError> {
    let bytes = URL_SAFE_NO_PAD
        .decode(s)
        .map_err(|_| InputError::new("cursor", "invalid cursor encoding"))?;
    serde_json::from_slice(&bytes).map_err(|_| InputError::new("cursor", "invalid cursor payload"))
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct FakeCursor {
        created_at: DateTime<Utc>,
        id: Uuid,
    }

    #[test]
    fn round_trip_preserves_value() {
        let original = FakeCursor {
            created_at: DateTime::parse_from_rfc3339("2026-05-18T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            id: Uuid::new_v4(),
        };
        let encoded = encode(&original);
        let decoded: FakeCursor = decode(&encoded).expect("round-trip should succeed");
        assert_eq!(decoded, original);
    }

    #[test]
    fn encoded_form_is_url_safe() {
        let c = FakeCursor {
            created_at: Utc::now(),
            id: Uuid::new_v4(),
        };
        let encoded = encode(&c);
        assert!(
            encoded
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'),
            "encoded cursor contains non-url-safe chars: {encoded}"
        );
    }

    #[test]
    fn rejects_garbage_input() {
        let result: Result<FakeCursor, _> = decode("not!valid#base64");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_valid_base64_with_wrong_payload() {
        let bogus = URL_SAFE_NO_PAD.encode(b"\"hello\"");
        let result: Result<FakeCursor, _> = decode(&bogus);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_input() {
        let result: Result<FakeCursor, _> = decode("");
        assert!(result.is_err());
    }
}
