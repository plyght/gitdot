-- Re-key verification codes to (user_id, email): a secondary email is no
-- longer pre-inserted into core.user_emails, so codes can no longer reference a
-- user_email_id. In-flight codes are disposable, so drop and recreate.
DROP TABLE auth.email_verification_codes;

CREATE TABLE auth.email_verification_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES core.users(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    code_hash VARCHAR(128) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ
);

CREATE INDEX idx_email_verification_codes_user_email
    ON auth.email_verification_codes (user_id, email);
