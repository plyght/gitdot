-- Track failed verification attempts per login code so a code can be locked out
-- after a small number of wrong guesses, instead of being grindable up to its
-- full expiry window.
ALTER TABLE auth.auth_codes ADD COLUMN attempt_count SMALLINT NOT NULL DEFAULT 0;
