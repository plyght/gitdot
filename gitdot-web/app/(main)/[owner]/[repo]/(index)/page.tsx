import type { RepositoryBlobResource } from "gitdot-api";
import { getRepository } from "@/dal/repository";
import { fetchResources } from "@/provider/server";
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

  const { requests, promises } = fetchResources(owner, repo, {
    readme: (p) => p.getBlob("README.md"),
  });

  return (
    <div className="flex h-full w-full overflow-hidden">
      <PageClient
        owner={owner}
        repo={repo}
        requests={requests}
        promises={promises}
      />
      <RepoPanel repository={repository} />
    </div>
  );
}
