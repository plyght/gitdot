CREATE TABLE core.user_emails (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    verified_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX idx_user_emails_email ON core.user_emails (email);

CREATE UNIQUE INDEX idx_user_emails_one_primary
    ON core.user_emails (user_id) WHERE is_primary;

CREATE INDEX idx_user_emails_verified_lookup
    ON core.user_emails (email) WHERE is_verified;

INSERT INTO core.user_emails (user_id, email, is_primary, is_verified, verified_at)
SELECT id, email, TRUE, is_email_verified,
       CASE WHEN is_email_verified THEN created_at ELSE NULL END
FROM core.users;
