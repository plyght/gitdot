"use client";

import type {
  GetRepositoryActivityResponse,
  RepositoryResource,
} from "gitdot-api";
import { useRightSidebar } from "@/(main)/hooks/use-sidebar";
import { RepoActions } from "./repo-actions";
import { RepoActivity } from "./repo-activity";
import { RepoInfo } from "./repo-info";

export function RepoPanel({
  repository,
  activityPromise,
}: {
  repository: RepositoryResource;
  activityPromise: Promise<GetRepositoryActivityResponse | null>;
}) {
  const open = useRightSidebar();
  if (!open) return null;

  return (
    <div className="w-64 shrink-0 h-full border-l flex flex-col">
      <RepoInfo repository={repository} />
      <RepoActions repository={repository} />
      <RepoActivity activityPromise={activityPromise} />
    </div>
  );
}
