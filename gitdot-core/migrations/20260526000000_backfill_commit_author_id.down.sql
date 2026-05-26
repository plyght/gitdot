DROP TRIGGER IF EXISTS link_commits_on_email_verified ON core.user_emails;
DROP FUNCTION IF EXISTS core.link_commits_to_verified_email();
DROP INDEX IF EXISTS core.idx_commits_git_author_email_unmatched;
