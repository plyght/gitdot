import type { RepositoryBlobResource } from "gitdot-api";
import {
  getCurrentUser,
  getRepository,
  getRepositoryActivity,
} from "gitdot-client";
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
  const [repository, current] = await Promise.all([
    getRepository(owner, repo),
    getCurrentUser(false),
  ]);
  if (!repository) return null;

  const isAdmin =
    current?.name === owner ||
    (current?.memberships ?? []).some(
      (m) => m.name === owner && m.role === "admin",
    );

  const resources = fetchResources({
    readme: (p) => p.getBlob(owner, repo, "README.md"),
  });

  const activityPromise = getRepositoryActivity(owner, repo);

  return (
    <div className="flex h-full w-full overflow-hidden">
      <PageClient resources={resources} />
      <RepoPanel
        repository={repository}
        activityPromise={activityPromise}
        currentUser={current ?? null}
        isAdmin={isAdmin}
      />
    </div>
  );
}
