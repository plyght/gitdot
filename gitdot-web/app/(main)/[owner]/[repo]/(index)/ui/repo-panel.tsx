import type { RepositoryResource } from "gitdot-api";
import { RepoActions } from "./repo-actions";
import { RepoActivity } from "./repo-activity";
import { RepoInfo } from "./repo-info";

export function RepoPanel({
  repositoryPromise,
}: {
  repositoryPromise: Promise<RepositoryResource | null>;
}) {
  return (
    <div className="w-64 shrink-0 h-full border-l flex flex-col">
      <RepoInfo repositoryPromise={repositoryPromise} />
      <RepoActions />
      <RepoActivity />
    </div>
  );
}
