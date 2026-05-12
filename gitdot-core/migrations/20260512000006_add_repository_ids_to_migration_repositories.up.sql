ALTER TABLE migration.migration_repositories
    ADD COLUMN origin_repository_id BIGINT NOT NULL;

ALTER TABLE migration.migration_repositories
    ADD COLUMN destination_repository_id UUID
        REFERENCES core.repositories(id) ON DELETE SET NULL;

CREATE INDEX idx_migration_repositories_origin_repository_id
    ON migration.migration_repositories (origin_repository_id);
