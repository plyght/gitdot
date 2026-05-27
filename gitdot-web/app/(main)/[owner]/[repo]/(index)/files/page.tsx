import type { RepositoryPathsResource } from "gitdot-api";
import { fetchResources } from "gitdot-dal/server";
import { PageClient } from "./page.client";

export type Resources = {
  paths: RepositoryPathsResource | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const resources = fetchResources(owner, repo, {
    paths: (p) => p.getPaths(),
  });

  return <PageClient owner={owner} repo={repo} resources={resources} />;
}
