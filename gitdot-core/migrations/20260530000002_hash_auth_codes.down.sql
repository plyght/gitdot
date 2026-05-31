TRUNCATE TABLE auth.auth_codes;

DROP INDEX IF EXISTS auth.idx_auth_codes_code_hash;
CREATE INDEX idx_auth_codes_user_code ON auth.auth_codes (code_hash);

ALTER TABLE auth.auth_codes ALTER COLUMN code_hash TYPE VARCHAR(16);
ALTER TABLE auth.auth_codes RENAME COLUMN code_hash TO user_code;
