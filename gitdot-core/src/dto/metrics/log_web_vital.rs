use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LogWebVitalRequest {
    pub events: Vec<WebVitalEvent>,

    pub user_id: Option<Uuid>,
    pub ip: String,
    pub user_agent: String,

    pub country: String,
    pub region: String,
    pub city: String,
}

#[derive(Debug, Clone)]
pub struct WebVitalEvent {
    pub event_time: DateTime<Utc>,
    pub name: String,
    pub value: f64,
    pub rating: String,
    pub metric_id: String,
    pub navigation_type: Option<String>,
    pub route: String,
    pub path: String,
}
