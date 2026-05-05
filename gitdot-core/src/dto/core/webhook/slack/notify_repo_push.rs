use serde::{Deserialize, Serialize};

use crate::dto::RepoPushCommit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyRepoPushRequest {
    pub channel_id: String,
    pub owner: String,
    pub repo: String,
    pub ref_name: String,
    pub pusher_name: String,
    pub commits: Vec<RepoPushCommit>,
}
