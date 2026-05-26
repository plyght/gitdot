CREATE INDEX idx_commits_git_author_email_unmatched
    ON core.commits (git_author_email)
    WHERE author_id IS NULL;

UPDATE core.commits c
SET author_id = ue.user_id
FROM core.user_emails ue
WHERE c.author_id IS NULL
  AND c.git_author_email = ue.email
  AND ue.is_verified;

CREATE OR REPLACE FUNCTION core.link_commits_to_verified_email()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_verified AND (
        TG_OP = 'INSERT'
        OR OLD.is_verified IS DISTINCT FROM NEW.is_verified
        OR OLD.email IS DISTINCT FROM NEW.email
    ) THEN
        UPDATE core.commits
        SET author_id = NEW.user_id
        WHERE author_id IS NULL
          AND git_author_email = NEW.email;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER link_commits_on_email_verified
    AFTER INSERT OR UPDATE OF is_verified, email ON core.user_emails
    FOR EACH ROW
    EXECUTE FUNCTION core.link_commits_to_verified_email();
