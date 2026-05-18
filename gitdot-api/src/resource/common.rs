use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::ApiResource;

/// Shared response envelope for paginated list endpoints.
///
/// `next_cursor` is `None` (omitted from the JSON via `skip_serializing_if`)
/// on the final page. Clients pass it back as `?cursor=...` to fetch the
/// next page; the encoding is opaque.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl<T: ApiResource + DeserializeOwned> ApiResource for Page<T> {}
