import {
  getCurrentUser,
  getRepository,
  getRepositoryActivity,
  getRepositoryBlob,
} from "gitdot-client";
import { MarkdownBody } from "../ui/markdown/markdown-body";
import { RepoPanel } from "./ui/repo-panel";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const [repository, current, readme] = await Promise.all([
    getRepository(owner, repo),
    getCurrentUser(false),
    getRepositoryBlob(owner, repo, { path: "README.md" }),
  ]);
  if (!repository) return null;

  const isAdmin =
    current?.name === owner ||
    (current?.memberships ?? []).some(
      (m) => m.name === owner && m.role === "admin",
    );

  const activityPromise = getRepositoryActivity(owner, repo);

  return (
    <div className="flex h-full w-full overflow-hidden">
      <div className="flex-1 min-w-0 overflow-y-auto scrollbar-none">
        {readme ? (
          <div className="p-4 w-full">
            <MarkdownBody content={readme.content} />
          </div>
        ) : (
          <div className="p-2 font-mono h-9 text-sm">README.md not found</div>
        )}
      </div>
      <RepoPanel
        repository={repository}
        activityPromise={activityPromise}
        currentUser={current ?? null}
        isAdmin={isAdmin}
      />
    </div>
  );
}
