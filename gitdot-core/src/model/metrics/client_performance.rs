use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, clickhouse::Row, serde::Serialize, serde::Deserialize)]
pub struct ClientPerformance {
    // metric
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub event_time: DateTime<Utc>,
    pub name: String,
    pub value: f64,
    pub rating: String,
    pub metric_id: String,
    pub navigation_type: String,

    // page
    pub route: String,
    pub path: String,

    // geo
    pub country: String,
    pub region: String,
    pub city: String,

    // user
    #[serde(with = "clickhouse::serde::uuid::option")]
    pub user_id: Option<Uuid>,
    pub ip: String,
    pub user_agent: String,
}
