-- Track failed verification attempts per email verification code so a code can
-- be locked out after a small number of wrong guesses, instead of being
-- grindable up to its full expiry window. Mirrors auth.auth_codes.attempt_count.
ALTER TABLE auth.email_verification_codes ADD COLUMN attempt_count SMALLINT NOT NULL DEFAULT 0;
