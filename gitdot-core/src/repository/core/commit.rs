use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{Commit, CommitDiff},
};

const COMMIT_PROJECTION_QUERY: &str = "
    c.id, c.repo_id, c.author_id, c.git_author_name, c.git_author_email,
    c.ref_name, c.sha, c.parent_sha, c.message, c.created_at,
    c.review_number, c.diff_position, c.diffs,
    au.name AS author_name,
    au.image_updated_at AS author_image_updated_at,
    json_build_object(
        'id',         r.id,
        'owner_name', COALESCE(u.name, o.name),
        'name',       r.name,
        'visibility', r.visibility
    ) AS repository
";

const COMMIT_JOINS_QUERY: &str = "
    JOIN core.repositories r ON c.repo_id = r.id
    LEFT JOIN core.users u
      ON r.owner_id = u.id AND r.owner_type = 'user'
    LEFT JOIN core.organizations o
      ON r.owner_id = o.id AND r.owner_type = 'organization'
    LEFT JOIN core.users au ON au.id = c.author_id
";

/// sqlx data-access layer for the `core.commits` table. Reads join to
/// `core.repositories` (and `core.users`/`core.organizations` for owner name,
/// `core.users` for the author) to hydrate each `Commit` with its embedded
/// `repository` object and author display fields.
#[async_trait]
pub trait CommitRepository: Send + Sync + Clone + 'static {
    /// Returns the commit in `repo_id` matching `sha`. Matches against the
    /// generated `sha_short` column using the first 7 chars of `sha` (or the
    /// whole string if shorter), so both full and abbreviated SHAs resolve.
    /// `Ok(None)` if no commit matches in that repo.
    async fn get_commit(&self, repo_id: Uuid, sha: &str) -> Result<Option<Commit>, DatabaseError>;

    /// Lists commits in `repo_id` on `ref_name` within the `[from, to]`
    /// `created_at` window, newest first (`ORDER BY created_at DESC, id DESC`),
    /// cursor-paginated. Returns the page plus the next `Cursor` (`None` on the
    /// last page).
    async fn list_by_repository(
        &self,
        repo_id: Uuid,
        ref_name: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Commit>, Option<Cursor>), DatabaseError>;

    /// Lists commits authored by `author_id` within the `[from, to]`
    /// `created_at` window, newest first, cursor-paginated. Each row carries a
    /// computed `viewer_has_access` bool: true when the commit's repo is public,
    /// owned by `viewer_id`, or owned by an org `viewer_id` belongs to (via
    /// `core.organization_members`). Returns `(Commit, access)` pairs plus the
    /// next `Cursor`.
    async fn list_by_user(
        &self,
        author_id: Uuid,
        viewer_id: Option<Uuid>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<(Commit, bool)>, Option<Cursor>), DatabaseError>;

    /// Bulk-inserts commits into `core.commits` from parallel column arrays
    /// (`UNNEST`), with `ON CONFLICT (repo_id, sha) DO NOTHING`. Returns the
    /// hydrated rows that were actually inserted (conflicting SHAs are skipped
    /// and absent from the result). Empty `shas` is a no-op returning an empty
    /// `Vec`. Caller must supply equal-length slices.
    async fn create_bulk(
        &self,
        author_ids: &[Option<Uuid>],
        git_author_names: &[String],
        git_author_emails: &[String],
        repo_ids: &[Uuid],
        ref_names: &[String],
        shas: &[String],
        parent_shas: &[String],
        messages: &[String],
        created_ats: &[DateTime<Utc>],
        diffs: &[Vec<CommitDiff>],
        review_numbers: &[Option<i32>],
        diff_positions: &[Option<i32>],
    ) -> Result<Vec<Commit>, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct PgCommitRepository {
    pool: PgPool,
}

impl PgCommitRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl CommitRepository for PgCommitRepository {
    async fn get_commit(&self, repo_id: Uuid, sha: &str) -> Result<Option<Commit>, DatabaseError> {
        let short = if sha.len() >= 7 { &sha[..7] } else { sha };

        let query = format!(
            "SELECT {projection}
             FROM core.commits c
             {joins}
             WHERE c.repo_id = $1 AND c.sha_short = $2",
            projection = COMMIT_PROJECTION_QUERY,
            joins = COMMIT_JOINS_QUERY,
        );

        let commit = sqlx::query_as::<_, Commit>(&query)
            .bind(repo_id)
            .bind(short)
            .fetch_optional(&self.pool)
            .await?;

        Ok(commit)
    }

    async fn list_by_repository(
        &self,
        repo_id: Uuid,
        ref_name: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Commit>, Option<Cursor>), DatabaseError> {
        let query = format!(
            "SELECT {projection}
             FROM core.commits c
             {joins}
             WHERE c.repo_id = $1
               AND c.ref_name = $2
               AND c.created_at >= $3 AND c.created_at <= $4
               AND ($5::timestamptz IS NULL OR (c.created_at, c.id) < ($5, $6))
             ORDER BY c.created_at DESC, c.id DESC
             LIMIT $7",
            projection = COMMIT_PROJECTION_QUERY,
            joins = COMMIT_JOINS_QUERY,
        );

        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut commits = sqlx::query_as::<_, Commit>(&query)
            .bind(repo_id)
            .bind(ref_name)
            .bind(from)
            .bind(to)
            .bind(cursor_created_at)
            .bind(cursor_id)
            .bind(limit + 1)
            .fetch_all(&self.pool)
            .await?;

        let next_cursor = if commits.len() as i64 > limit {
            commits.pop();
            commits.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((commits, next_cursor))
    }

    async fn list_by_user(
        &self,
        author_id: Uuid,
        viewer_id: Option<Uuid>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<(Commit, bool)>, Option<Cursor>), DatabaseError> {
        #[derive(FromRow)]
        struct UserCommitRow {
            #[sqlx(flatten)]
            commit: Commit,
            viewer_has_access: bool,
        }

        let query = format!(
            "WITH viewer_orgs AS (
                SELECT organization_id FROM core.organization_members WHERE user_id = $7
             )
             SELECT
                {projection},
                COALESCE(
                    r.visibility = 'public'
                    OR (r.owner_type = 'user' AND r.owner_id = $7)
                    OR (r.owner_type = 'organization'
                        AND r.owner_id IN (SELECT organization_id FROM viewer_orgs)),
                    false
                ) AS viewer_has_access
             FROM core.commits c
             {joins}
             WHERE c.author_id = $1
               AND c.created_at >= $2 AND c.created_at <= $3
               AND ($4::timestamptz IS NULL OR (c.created_at, c.id) < ($4, $5))
             ORDER BY c.created_at DESC, c.id DESC
             LIMIT $6",
            projection = COMMIT_PROJECTION_QUERY,
            joins = COMMIT_JOINS_QUERY,
        );

        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut rows = sqlx::query_as::<_, UserCommitRow>(&query)
            .bind(author_id)
            .bind(from)
            .bind(to)
            .bind(cursor_created_at)
            .bind(cursor_id)
            .bind(limit + 1)
            .bind(viewer_id)
            .fetch_all(&self.pool)
            .await?;

        let next_cursor = if rows.len() as i64 > limit {
            rows.pop();
            rows.last().map(|last| Cursor {
                created_at: last.commit.created_at,
                id: last.commit.id,
            })
        } else {
            None
        };

        let data = rows
            .into_iter()
            .map(|r| (r.commit, r.viewer_has_access))
            .collect();

        Ok((data, next_cursor))
    }

    async fn create_bulk(
        &self,
        author_ids: &[Option<Uuid>],
        git_author_names: &[String],
        git_author_emails: &[String],
        repo_ids: &[Uuid],
        ref_names: &[String],
        shas: &[String],
        parent_shas: &[String],
        messages: &[String],
        created_ats: &[DateTime<Utc>],
        diffs: &[Vec<CommitDiff>],
        review_numbers: &[Option<i32>],
        diff_positions: &[Option<i32>],
    ) -> Result<Vec<Commit>, DatabaseError> {
        if shas.is_empty() {
            return Ok(Vec::new());
        }

        let diffs_json: Vec<serde_json::Value> = diffs
            .iter()
            .map(|d| serde_json::to_value(d).unwrap_or(serde_json::Value::Array(vec![])))
            .collect();

        let query = format!(
            r#"
            WITH inserted AS (
                INSERT INTO core.commits (
                    author_id, git_author_name, git_author_email, repo_id,
                    ref_name, sha, parent_sha, message, created_at, diffs,
                    review_number, diff_position
                )
                SELECT * FROM UNNEST(
                    $1::uuid[], $2::text[], $3::text[], $4::uuid[],
                    $5::varchar[], $6::varchar[], $7::varchar[], $8::text[],
                    $9::timestamptz[], $10::jsonb[], $11::int[], $12::int[]
                )
                ON CONFLICT (repo_id, sha) DO NOTHING
                RETURNING *
            )
            SELECT {projection}
            FROM inserted c
            {joins}
            "#,
            projection = COMMIT_PROJECTION_QUERY,
            joins = COMMIT_JOINS_QUERY,
        );

        let rows = sqlx::query_as::<_, Commit>(&query)
            .bind(author_ids)
            .bind(git_author_names)
            .bind(git_author_emails)
            .bind(repo_ids)
            .bind(ref_names)
            .bind(shas)
            .bind(parent_shas)
            .bind(messages)
            .bind(created_ats)
            .bind(diffs_json)
            .bind(review_numbers)
            .bind(diff_positions)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use std::collections::HashMap;

    use chrono::{Duration, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{Commit, CommitDiff, CommitRepository, PgCommitRepository};
    use crate::{
        model::OrganizationRole,
        repository::test_common::{
            insert_commit_on, insert_membership_at, insert_org, insert_org_repo, insert_user,
            insert_user_repo,
        },
    };

    fn access_by_repo(rows: Vec<(Commit, bool)>) -> HashMap<String, bool> {
        rows.into_iter()
            .map(|(c, access)| (c.repository.name, access))
            .collect()
    }

    #[sqlx::test]
    async fn get_commit_matches_full_and_short_sha(pool: PgPool) {
        let repo = PgCommitRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        let repo_id = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user_repo(&pool, repo_id, "proj", alice, "public").await;
        let sha = "a".repeat(40);
        insert_commit_on(
            &pool,
            repo_id,
            Some(alice),
            &sha,
            "refs/heads/main",
            Utc::now(),
        )
        .await;

        // A full 40-char sha resolves via the generated 7-char prefix.
        let full = repo
            .get_commit(repo_id, &sha)
            .await
            .unwrap()
            .expect("found by full sha");
        assert_eq!(full.sha, sha);
        assert_eq!(full.author_name.as_deref(), Some("alice"));
        assert_eq!(full.repository.owner_name, "alice");
        assert_eq!(full.repository.name, "proj");
        assert_eq!(full.repository.visibility, "public");

        // A 7-char prefix resolves the same commit.
        let short = repo
            .get_commit(repo_id, &sha[..7])
            .await
            .unwrap()
            .expect("found by short sha");
        assert_eq!(short.id, full.id);

        // A different repo or an unknown sha yields nothing.
        assert!(
            repo.get_commit(Uuid::new_v4(), &sha)
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            repo.get_commit(repo_id, &"b".repeat(40))
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn list_by_repository_filters_and_paginates(pool: PgPool) {
        let repo = PgCommitRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        let repo_id = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user_repo(&pool, repo_id, "proj", alice, "public").await;

        let now = Utc::now();
        insert_commit_on(
            &pool,
            repo_id,
            Some(alice),
            &"a".repeat(40),
            "refs/heads/main",
            now - Duration::days(3),
        )
        .await;
        insert_commit_on(
            &pool,
            repo_id,
            Some(alice),
            &"b".repeat(40),
            "refs/heads/main",
            now - Duration::days(2),
        )
        .await;
        insert_commit_on(
            &pool,
            repo_id,
            Some(alice),
            &"c".repeat(40),
            "refs/heads/main",
            now - Duration::days(1),
        )
        .await;
        // A commit on another branch is excluded by the ref filter.
        insert_commit_on(
            &pool,
            repo_id,
            Some(alice),
            &"d".repeat(40),
            "refs/heads/dev",
            now,
        )
        .await;
        // A commit outside the time window is excluded.
        insert_commit_on(
            &pool,
            repo_id,
            Some(alice),
            &"e".repeat(40),
            "refs/heads/main",
            now - Duration::days(100),
        )
        .await;

        let from = now - Duration::days(10);
        let to = now;
        let (page, cursor) = repo
            .list_by_repository(repo_id, "refs/heads/main", from, to, None, 2)
            .await
            .unwrap();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].sha, "c".repeat(40));
        assert_eq!(page[1].sha, "b".repeat(40));
        let cursor = cursor.expect("more rows remain");

        let (page, cursor) = repo
            .list_by_repository(repo_id, "refs/heads/main", from, to, Some(cursor), 2)
            .await
            .unwrap();
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].sha, "a".repeat(40));
        assert!(cursor.is_none());
    }

    #[sqlx::test]
    async fn list_by_user_returns_authored_commits_newest_first(pool: PgPool) {
        let repo = PgCommitRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        let repo_id = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;
        insert_user_repo(&pool, repo_id, "proj", alice, "public").await;

        let now = Utc::now();
        insert_commit_on(
            &pool,
            repo_id,
            Some(alice),
            &"a".repeat(40),
            "refs/heads/main",
            now - Duration::days(3),
        )
        .await;
        insert_commit_on(
            &pool,
            repo_id,
            Some(alice),
            &"b".repeat(40),
            "refs/heads/main",
            now - Duration::days(2),
        )
        .await;
        insert_commit_on(
            &pool,
            repo_id,
            Some(alice),
            &"c".repeat(40),
            "refs/heads/main",
            now - Duration::days(1),
        )
        .await;
        // bob's commit must not appear in alice's history.
        insert_commit_on(
            &pool,
            repo_id,
            Some(bob),
            &"d".repeat(40),
            "refs/heads/main",
            now,
        )
        .await;
        // alice's commit outside the window is excluded.
        insert_commit_on(
            &pool,
            repo_id,
            Some(alice),
            &"e".repeat(40),
            "refs/heads/main",
            now - Duration::days(100),
        )
        .await;

        let from = now - Duration::days(10);
        let to = now;
        let (page, cursor) = repo
            .list_by_user(alice, Some(alice), from, to, None, 2)
            .await
            .unwrap();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].0.sha, "c".repeat(40));
        assert_eq!(page[1].0.sha, "b".repeat(40));
        assert!(page[0].1, "owner viewer should have access");
        let cursor = cursor.expect("more rows remain");

        let (page, cursor) = repo
            .list_by_user(alice, Some(alice), from, to, Some(cursor), 2)
            .await
            .unwrap();
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].0.sha, "a".repeat(40));
        assert!(cursor.is_none());
    }

    #[sqlx::test]
    async fn list_by_user_reflects_viewer_access(pool: PgPool) {
        let repo = PgCommitRepository::new(pool.clone());
        let alice = Uuid::new_v4(); // commit author + owner of the user repos
        let bob = Uuid::new_v4(); // unrelated outsider
        let member = Uuid::new_v4(); // member of the org that owns org_repo
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;
        insert_user(&pool, member, "member").await;

        let public_repo = Uuid::new_v4();
        let private_repo = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let org_repo = Uuid::new_v4();
        insert_user_repo(&pool, public_repo, "pub", alice, "public").await;
        insert_user_repo(&pool, private_repo, "priv", alice, "private").await;
        insert_org(&pool, org_id, "acme").await;
        insert_org_repo(&pool, org_repo, "orgproj", org_id, "private").await;
        insert_membership_at(&pool, member, org_id, OrganizationRole::Member, Utc::now()).await;

        let now = Utc::now();
        // alice authors one commit in each repo.
        insert_commit_on(
            &pool,
            public_repo,
            Some(alice),
            &"a".repeat(40),
            "refs/heads/main",
            now - Duration::days(1),
        )
        .await;
        insert_commit_on(
            &pool,
            private_repo,
            Some(alice),
            &"b".repeat(40),
            "refs/heads/main",
            now - Duration::days(2),
        )
        .await;
        insert_commit_on(
            &pool,
            org_repo,
            Some(alice),
            &"c".repeat(40),
            "refs/heads/main",
            now - Duration::days(3),
        )
        .await;

        let from = now - Duration::days(10);
        let to = now;

        // The owner sees the public repo (visibility) and their own private repo,
        // but not the org repo they don't belong to.
        let owner = access_by_repo(
            repo.list_by_user(alice, Some(alice), from, to, None, 50)
                .await
                .unwrap()
                .0,
        );
        assert!(owner["pub"]);
        assert!(owner["priv"]);
        assert!(!owner["orgproj"]);

        // An outsider sees only the public repo.
        let outsider = access_by_repo(
            repo.list_by_user(alice, Some(bob), from, to, None, 50)
                .await
                .unwrap()
                .0,
        );
        assert!(outsider["pub"]);
        assert!(!outsider["priv"]);
        assert!(!outsider["orgproj"]);

        // An org member gains access to the org repo via membership.
        let org_member = access_by_repo(
            repo.list_by_user(alice, Some(member), from, to, None, 50)
                .await
                .unwrap()
                .0,
        );
        assert!(org_member["pub"]);
        assert!(!org_member["priv"]);
        assert!(org_member["orgproj"]);

        // An anonymous viewer sees only the public repo.
        let anon = access_by_repo(
            repo.list_by_user(alice, None, from, to, None, 50)
                .await
                .unwrap()
                .0,
        );
        assert!(anon["pub"]);
        assert!(!anon["priv"]);
        assert!(!anon["orgproj"]);
    }

    #[sqlx::test]
    async fn create_bulk_inserts_and_skips_conflicts(pool: PgPool) {
        let repo = PgCommitRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        let repo_id = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user_repo(&pool, repo_id, "proj", alice, "public").await;

        let now = Utc::now();
        let sha_a = "a".repeat(40);
        let sha_b = "b".repeat(40);
        let zero = "0".repeat(40);
        let diffs_a = vec![CommitDiff {
            path: "src/lib.rs".to_string(),
            lines_added: 10,
            lines_removed: 2,
        }];

        let inserted = repo
            .create_bulk(
                &[Some(alice), Some(alice)],
                &["Alice".to_string(), "Alice".to_string()],
                &[
                    "alice@example.com".to_string(),
                    "alice@example.com".to_string(),
                ],
                &[repo_id, repo_id],
                &["refs/heads/main".to_string(), "refs/heads/main".to_string()],
                &[sha_a.clone(), sha_b.clone()],
                &[zero.clone(), sha_a.clone()],
                &["first".to_string(), "second".to_string()],
                &[now - Duration::days(1), now],
                &[diffs_a.clone(), vec![]],
                &[None, None],
                &[None, None],
            )
            .await
            .unwrap();
        assert_eq!(inserted.len(), 2);
        let a = inserted
            .iter()
            .find(|c| c.sha == sha_a)
            .expect("sha_a present");
        assert_eq!(a.message, "first");
        assert_eq!(a.author_name.as_deref(), Some("alice"));
        assert_eq!(a.repository.name, "proj");
        assert_eq!(a.diffs.len(), 1);
        assert_eq!(a.diffs[0].path, "src/lib.rs");

        // Re-running with one existing sha and one new sha inserts only the new one.
        let sha_c = "c".repeat(40);
        let again = repo
            .create_bulk(
                &[Some(alice), Some(alice)],
                &["Alice".to_string(), "Alice".to_string()],
                &[
                    "alice@example.com".to_string(),
                    "alice@example.com".to_string(),
                ],
                &[repo_id, repo_id],
                &["refs/heads/main".to_string(), "refs/heads/main".to_string()],
                &[sha_a.clone(), sha_c.clone()],
                &[zero.clone(), sha_b.clone()],
                &["dup".to_string(), "third".to_string()],
                &[now, now],
                &[vec![], vec![]],
                &[None, None],
                &[None, None],
            )
            .await
            .unwrap();
        assert_eq!(again.len(), 1);
        assert_eq!(again[0].sha, sha_c);

        // Empty input is a no-op.
        let empty = repo
            .create_bulk(&[], &[], &[], &[], &[], &[], &[], &[], &[], &[], &[], &[])
            .await
            .unwrap();
        assert!(empty.is_empty());
    }
}
