use std::{sync::Arc, time::Duration};

use governor::middleware::NoOpMiddleware;
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};

/// How often the background task evicts stale per-IP state from a limiter.
const CLEANUP_INTERVAL: Duration = Duration::from_secs(60);

/// Build a per-IP [`GovernorLayer`] that replenishes one token every `period`
/// with an initial `burst`, so sustained throughput ≈ `1s / period`.
///
/// A background task is spawned to periodically evict stale per-IP entries so the
/// limiter's map can't grow unbounded over the process lifetime.
pub fn create_rate_limiter(
    period: Duration,
    burst: u32,
) -> GovernorLayer<SmartIpKeyExtractor, NoOpMiddleware> {
    let config = Arc::new(
        GovernorConfigBuilder::default()
            .period(period)
            .burst_size(burst)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .expect("valid governor config"),
    );

    let limiter = config.limiter().clone();
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(CLEANUP_INTERVAL);
        loop {
            tick.tick().await;
            limiter.retain_recent();
        }
    });

    GovernorLayer { config }
}
