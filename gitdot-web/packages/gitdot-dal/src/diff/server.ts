import "server-only";

import type { RepositoryDiffFileResource } from "gitdot-api";
import {
  getRepositoryBlobDiffs,
  getRepositoryCommitDiff,
  getReviewDiff,
} from "gitdot-client";
import { renderDiff } from "./shiki";
import type { DiffEntry } from "./types";

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
  const entries = await Promise.all(
    Object.entries(result.diffs).map(
      async ([sha, file]) =>
        [sha, { resource: file, spans: await renderDiff(file) }] as const,
    ),
  );
  return Object.fromEntries(entries);
}

export async function renderCommitDiff(
  owner: string,
  repo: string,
  sha: string,
): Promise<DiffEntry[]> {
  const result = await getRepositoryCommitDiff(owner, repo, sha);
  if (!result) return [];
  return renderDiffs(result.files);
}

export async function renderReviewDiff(
  owner: string,
  repo: string,
  number: number | string,
  position: number,
  revision?: number,
  compareTo?: number,
): Promise<DiffEntry[]> {
  const result = await getReviewDiff(
    owner,
    repo,
    number,
    position,
    revision,
    compareTo,
  );
  if (!result) return [];
  return renderDiffs(result.files);
}

async function renderDiffs(
  files: RepositoryDiffFileResource[],
): Promise<DiffEntry[]> {
  const datas = await Promise.all(files.map(renderDiff));
  return files.map((file, i) => ({ resource: file, spans: datas[i] }));
}
