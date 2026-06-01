import { getRepositoryCommit } from "gitdot-client";
import type { DiffData } from "gitdot-dal/client";
import { renderCommitDiff } from "gitdot-dal/server";
import { notFound } from "next/navigation";
import { PageClient } from "./page.client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; sha: string }>;
}) {
  const { owner, repo, sha } = await params;

  const commit = await getRepositoryCommit(owner, repo, sha);
  if (!commit) notFound();

  const diffEntriesPromise: Promise<DiffData> = renderCommitDiff(
    owner,
    repo,
    sha,
  );

  return (
    <PageClient
      owner={owner}
      repo={repo}
      commit={commit}
      diffEntriesPromise={diffEntriesPromise}
    />
  );
}
