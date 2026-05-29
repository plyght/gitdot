-- Composite index for the repo commit-history access path in `list_by_repository`:
-- `WHERE repo_id = $1 AND ref_name = $2 AND created_at BETWEEN $3 AND $4
--  ORDER BY created_at DESC, id DESC LIMIT N` becomes an index scan that already
-- returns sorted rows, satisfying both the filter and the keyset-pagination order.
-- The existing single-column indexes (idx_commits_repo_id / _ref_name / _created_at)
-- could each serve only part of this query, forcing an extra sort.
CREATE INDEX idx_commits_repo_id_ref_name_created_at_id
ON core.commits (repo_id, ref_name, created_at DESC, id DESC);
