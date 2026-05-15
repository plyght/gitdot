import type {
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  commits: RepositoryCommitResource[] | null;
  settings: RepositorySettingsResource | null;
  paths: RepositoryPathsResource | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    commits: (p) => p.getCommits(),
    settings: (p) => p.getSettings(),
    paths: (p) => p.getPaths(),
  });

  return (
    <PageClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
    />
  );
}
