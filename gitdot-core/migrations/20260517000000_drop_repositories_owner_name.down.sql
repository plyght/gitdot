ALTER TABLE core.repositories ADD COLUMN owner_name VARCHAR(100) NOT NULL DEFAULT '';

UPDATE core.repositories r
SET owner_name = u.name
FROM core.users u
WHERE r.owner_id = u.id AND r.owner_type = 'user';

UPDATE core.repositories r
SET owner_name = o.name
FROM core.organizations o
WHERE r.owner_id = o.id AND r.owner_type = 'organization';

CREATE INDEX IF NOT EXISTS idx_repositories_owner_name ON core.repositories (owner_name);
CREATE INDEX IF NOT EXISTS idx_repositories_owner_name_name ON core.repositories (owner_name, name);
