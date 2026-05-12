import { RepoActions } from "./repo-actions";
import { RepoActivity } from "./repo-activity";

export function RepoPanel() {
  return (
    <div className="w-64 shrink-0 h-full border-l flex flex-col">
      <RepoActions />
      <RepoActivity />
    </div>
  );
}
