ALTER TABLE core.users ADD COLUMN image_updated_at TIMESTAMPTZ;
ALTER TABLE core.organizations ADD COLUMN image_updated_at TIMESTAMPTZ;

-- Existing users/orgs get a generated avatar at signup/creation, so seed the
-- version from created_at to give every avatar URL a stable, cacheable `?t=`.
UPDATE core.users SET image_updated_at = created_at;
UPDATE core.organizations SET image_updated_at = created_at;

-- New rows default to creation time; the value is bumped on each avatar upload.
ALTER TABLE core.users
    ALTER COLUMN image_updated_at SET DEFAULT now(),
    ALTER COLUMN image_updated_at SET NOT NULL;
ALTER TABLE core.organizations
    ALTER COLUMN image_updated_at SET DEFAULT now(),
    ALTER COLUMN image_updated_at SET NOT NULL;
