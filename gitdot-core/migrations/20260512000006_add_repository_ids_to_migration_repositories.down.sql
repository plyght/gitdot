DROP INDEX IF EXISTS migration.idx_migration_repositories_origin_repository_id;

ALTER TABLE migration.migration_repositories
    DROP COLUMN destination_repository_id;

ALTER TABLE migration.migration_repositories
    DROP COLUMN origin_repository_id;
