CREATE TABLE IF NOT EXISTS gitdot.client_performance (
    -- metric
    event_time      DateTime64(3, 'UTC') CODEC(DoubleDelta, LZ4),
    event_date      Date MATERIALIZED toDate(event_time),
    name            LowCardinality(String),
    value           Float64,
    rating          LowCardinality(String),
    metric_id       String,
    navigation_type LowCardinality(String) DEFAULT '',

    -- page
    route           LowCardinality(String) DEFAULT '',
    path            String,

    -- geo
    country         LowCardinality(String) DEFAULT '',
    region          LowCardinality(String) DEFAULT '',
    city            LowCardinality(String) DEFAULT '',

    -- user / client
    user_id         Nullable(UUID),
    ip              String DEFAULT '' CODEC(ZSTD(3)),
    user_agent      String DEFAULT '' CODEC(ZSTD(3)),

    -- ingestion
    ingested_at     DateTime64(3, 'UTC') DEFAULT now64(3) CODEC(DoubleDelta, LZ4)
) ENGINE = MergeTree
PARTITION BY toYYYYMM(event_date)
ORDER BY (name, route, event_time)
TTL event_date + INTERVAL 90 DAY;
