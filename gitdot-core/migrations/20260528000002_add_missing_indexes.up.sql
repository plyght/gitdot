-- FK index: deleting a core.repositories row triggers ON DELETE SET NULL on
-- migration.migration_repositories.destination_repository_id, which without an
-- index requires a sequential scan. Also supports list_by_origin_repository_id.
CREATE INDEX idx_migration_repositories_destination_repository_id
    ON migration.migration_repositories (destination_repository_id);

-- Supports list_trending: WHERE visibility = 'public' ORDER BY stars DESC,
-- created_at DESC. Partial index keeps it small (public repos only).
CREATE INDEX idx_repositories_trending
    ON core.repositories (stars DESC, created_at DESC)
    WHERE visibility = 'public';
