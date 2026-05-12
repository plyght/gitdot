CREATE SCHEMA IF NOT EXISTS webhook;

-- Postgres rebases the enum type's schema; the existing column types on
-- webhook tables follow automatically because they reference the same OID.
ALTER TYPE core.webhook_event_type SET SCHEMA webhook;

-- Indexes follow their tables automatically when SET SCHEMA is used.
ALTER TABLE core.webhooks SET SCHEMA webhook;
ALTER TABLE core.slack_webhooks SET SCHEMA webhook;
