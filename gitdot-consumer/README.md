# gitdot-consumer

Long-running Kafka consumer for the [gitdot](https://gitdot.io) backend. Subscribes to `gitdot.repo.pushed` events, looks up the Slack webhooks configured for each repo, and forwards push notifications to the gitdot Slack bot.

Like the other binary crates, this is a thin shell — message decoding and dispatch logic live in `WebhookService` in [`gitdot-core`](../gitdot-core).

## Semantics

- **At-least-once delivery** — offsets are committed only after the handler returns `Ok`, so failed messages are redelivered.
- **Sequential per partition** — one message is processed at a time; fan-out to multiple Slack channels happens inside the handler.
- **Graceful shutdown** — `SIGTERM` / `SIGINT` break the consume loop after the in-flight message completes.

## Run

```sh
cargo run -p gitdot-consumer
```

Configuration is loaded from environment variables (see `.env.example`). `KAFKA_AUTH` selects between local plaintext brokers and GCP Managed Kafka via SASL OAUTHBEARER.

## License

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0). Copyright © gitdot contributors.
