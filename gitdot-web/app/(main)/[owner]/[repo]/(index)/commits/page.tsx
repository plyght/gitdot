import type {
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { getRepository, listRepositoryCommitFilters } from "gitdot-client";
import { fetchResources } from "gitdot-dal/server";
import { PageClient } from "./page.client";

export type Resources = {
  commits: RepositoryCommitResource[] | null;
  paths: RepositoryPathsResource | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const resources = fetchResources(owner, repo, {
    commits: (p) => p.getCommits(),
    paths: (p) => p.getPaths(),
  });
  const [repository, commitFiltersResult] = await Promise.all([
    getRepository(owner, repo),
    listRepositoryCommitFilters(owner, repo),
  ]);
  const commitFilters = commitFiltersResult?.data ?? null;

  return (
    <PageClient
      owner={owner}
      repo={repo}
      resources={resources}
      repository={repository}
      commitFilters={commitFilters}
    />
  );
}
