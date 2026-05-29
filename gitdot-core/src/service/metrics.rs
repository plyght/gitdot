use async_trait::async_trait;

use crate::{
    client::ClickHouseClient, dto::LogWebVitalRequest, error::MetricsError,
    model::ClientPerformance,
};

const CLIENT_PERFORMANCE_TABLE: &str = "client_performance";

/// Ingestion of client-side observability data (web-vitals/performance events)
/// into the analytics store.
#[async_trait]
pub trait MetricsService: Send + Sync + 'static {
    /// Records a batch of client web-vital events into ClickHouse.
    ///
    /// Each event is enriched with the request-level context shared across the
    /// batch (geo location, user, IP, user agent) before insertion into the
    /// `client_performance` table.
    async fn log_web_vital(&self, request: LogWebVitalRequest) -> Result<(), MetricsError>;
}

pub struct MetricsServiceImpl<C: ClickHouseClient> {
    clickhouse: C,
}

impl<C: ClickHouseClient> MetricsServiceImpl<C> {
    pub fn new(clickhouse: C) -> Self {
        Self { clickhouse }
    }
}

#[async_trait]
impl<C: ClickHouseClient> MetricsService for MetricsServiceImpl<C> {
    async fn log_web_vital(&self, request: LogWebVitalRequest) -> Result<(), MetricsError> {
        let rows: Vec<ClientPerformance> = request
            .events
            .into_iter()
            .map(|ev| ClientPerformance {
                event_time: ev.event_time,
                name: ev.name,
                value: ev.value,
                rating: ev.rating,
                metric_id: ev.metric_id,
                navigation_type: ev.navigation_type.unwrap_or_default(),
                route: ev.route,
                path: ev.path,
                country: request.country.clone(),
                region: request.region.clone(),
                city: request.city.clone(),
                user_id: request.user_id,
                ip: request.ip.clone(),
                user_agent: request.user_agent.clone(),
            })
            .collect();

        // TODO: instead of directly inserting to clickhouse, send it to Kafka
        self.clickhouse
            .insert(CLIENT_PERFORMANCE_TABLE, &rows)
            .await?;

        Ok(())
    }
}
