import type { RepositoryCommitResource } from "gitdot-api";
import type { DiffEntry } from "gitdot-dal/client";
import { fetchResources } from "gitdot-dal/server";
import { PageClient } from "./page.client";

export type Resources = {
  commit: RepositoryCommitResource | null;
  diff: DiffEntry[];
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; sha: string }>;
}) {
  const { owner, repo, sha } = await params;

  const resources = fetchResources({
    commit: (p) => p.getCommit(owner, repo, sha),
    diff: (p) => p.getCommitDiff(owner, repo, sha),
  });

  return <PageClient owner={owner} repo={repo} resources={resources} />;
}
