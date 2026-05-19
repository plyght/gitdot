-- Composite index for keyset pagination on `list_by_user`:
-- `WHERE author_id = $1 ORDER BY created_at DESC, id DESC LIMIT N` becomes an
-- index scan that already returns sorted rows. Replaces use of the standalone
-- idx_commits_author_id for this access path (still useful for other queries).
CREATE INDEX idx_commits_author_id_created_at_id
ON core.commits (author_id, created_at DESC, id DESC);
