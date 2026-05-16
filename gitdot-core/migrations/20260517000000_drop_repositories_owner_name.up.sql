-- Drop the denormalized owner_name column on core.repositories.
-- owner_name is now sourced from JOINs to core.users / core.organizations
-- via COALESCE on owner_type.

-- Drop indexes that referenced the column.
DROP INDEX IF EXISTS core.idx_repositories_owner_name_name;
DROP INDEX IF EXISTS core.idx_repositories_owner_name;

-- Drop the column.
ALTER TABLE core.repositories DROP COLUMN owner_name;

-- The UNIQUE(owner_id, name) constraint from the original create migration
-- already provides the (owner_id, name) lookup index, so no new index is
-- needed for the inverse lookup path.
