import "server-only";

import type {
  RepositoryBlobPairResource,
  RepositoryBlobResource,
  RepositoryDiffFileResource,
} from "gitdot-api";
import {
  getRepositoryBlobDiffs,
  getRepositoryCommitBlobs,
  getReviewDiffBlobs,
} from "gitdot-client";
import { renderDiff } from "./shiki";
import type { DiffData, DiffEntry } from "./types";

/**
 * Adapt the legacy file-diff shape (left/right content strings) into a blob
 * pair. Transitional: the review and blob-diff endpoints still return
 * RepositoryDiffFileResource; drop this once they emit blob pairs natively.
 */
function fileToPair(
  file: RepositoryDiffFileResource,
): RepositoryBlobPairResource {
  const blob = (content: string): RepositoryBlobResource => ({
    path: file.path,
    content,
    commit_sha: "",
    sha: "",
    encoding: "utf-8",
  });
  return {
    path: file.path,
    old: file.left_content != null ? blob(file.left_content) : undefined,
    new: file.right_content != null ? blob(file.right_content) : undefined,
  };
}

export async function renderBlobDiffs(
  owner: string,
  repo: string,
  commitShas: string[],
  path: string,
): Promise<Record<string, DiffEntry>> {
  const result = await getRepositoryBlobDiffs(owner, repo, {
    commit_shas: commitShas,
    path,
  });
  if (!result) return {};
  const shas = Object.keys(result.diffs);
  const data = await renderDiff(
    shas.map((sha) => fileToPair(result.diffs[sha])),
  );
  return Object.fromEntries(shas.map((sha, i) => [sha, data[i]]));
}

export async function renderCommitDiff(
  owner: string,
  repo: string,
  sha: string,
): Promise<DiffData> {
  const pairs = await getRepositoryCommitBlobs(owner, repo, sha);
  if (!pairs) return [];
  return renderDiff(pairs);
}

export async function renderReviewDiff(
  owner: string,
  repo: string,
  number: number | string,
  position: number,
  revision?: number,
  compareTo?: number,
): Promise<DiffData> {
  const result = await getReviewDiffBlobs(
    owner,
    repo,
    number,
    position,
    revision,
    compareTo,
  );
  if (!result) return [];
  return renderDiff(result);
}
