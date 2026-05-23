# gitdot-metrics

Axum HTTP server that ingests product telemetry for [gitdot](https://gitdot.io). Today it accepts batched web-vital events from the frontend (CLS, LCP, INP, FCP, TTFB) and writes them to ClickHouse for analysis.

The crate is a thin transport layer — handlers extract request parameters and delegate to `MetricsService` in [`gitdot-core`](../gitdot-core). It runs as its own binary so the ingestion path can be deployed and scaled independently of the main API.

## Endpoints

- `POST /metrics/web-vital` — accepts a batch of web-vital events. Auth is optional (`Principal`); IP, user-agent, and geo headers (country/region/city) are recorded alongside each event.

## Run

```sh
cargo run -p gitdot-metrics
```

Reads configuration from environment variables. Required vars include `CLICKHOUSE_URL`, `CLICKHOUSE_USER`, `CLICKHOUSE_PASSWORD`, `CLICKHOUSE_DATABASE`, `GITDOT_PUBLIC_KEY`, and `VERCEL_OIDC_URL`. Defaults: port `8083`, `CLICKHOUSE_URL=http://localhost:8123`.

## License

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0). Copyright © gitdot contributors.
