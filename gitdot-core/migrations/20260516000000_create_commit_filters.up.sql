CREATE TABLE core.commit_filters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES core.repositories(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    authors TEXT[],
    tags TEXT[],
    paths TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(repository_id, name)
);

CREATE INDEX idx_commit_filters_repository_id ON core.commit_filters (repository_id);
