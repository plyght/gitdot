-- Auth codes were stored as plaintext 6-char user codes. Hash them at rest so a
-- DB read can't reveal live login codes. Existing plaintext codes can't be
-- re-hashed, so drop them (codes are short-lived and re-requestable).
TRUNCATE TABLE auth.auth_codes;

ALTER TABLE auth.auth_codes RENAME COLUMN user_code TO code_hash;
ALTER TABLE auth.auth_codes ALTER COLUMN code_hash TYPE VARCHAR(128);

DROP INDEX IF EXISTS auth.idx_auth_codes_user_code;
CREATE INDEX idx_auth_codes_code_hash ON auth.auth_codes (code_hash);
