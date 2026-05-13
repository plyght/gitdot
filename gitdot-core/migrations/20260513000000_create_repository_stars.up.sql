CREATE TABLE core.stars (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    repository_id UUID NOT NULL REFERENCES core.repositories(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, repository_id)
);

CREATE INDEX idx_stars_user_id ON core.stars (user_id);
CREATE INDEX idx_stars_repository_id ON core.stars (repository_id);
