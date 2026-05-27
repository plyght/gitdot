import type { RepositoryBlobResource } from "gitdot-api";
import { getRepository, getRepositoryActivity } from "gitdot-client";
import { fetchResources } from "gitdot-dal/server";
import { PageClient } from "./page.client";
import { RepoPanel } from "./ui/repo-panel";

export type Resources = {
  readme: RepositoryBlobResource | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const repository = await getRepository(owner, repo);
  if (!repository) return null;

  const resources = fetchResources({
    readme: (p) => p.getBlob(owner, repo, "README.md"),
  });

  const activityPromise = getRepositoryActivity(owner, repo);

  return (
    <div className="flex h-full w-full overflow-hidden">
      <PageClient resources={resources} />
      <RepoPanel repository={repository} activityPromise={activityPromise} />
    </div>
  );
}
