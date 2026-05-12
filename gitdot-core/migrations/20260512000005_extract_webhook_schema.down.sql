ALTER TABLE webhook.slack_webhooks SET SCHEMA core;
ALTER TABLE webhook.webhooks SET SCHEMA core;
ALTER TYPE webhook.webhook_event_type SET SCHEMA core;
DROP SCHEMA IF EXISTS webhook;
