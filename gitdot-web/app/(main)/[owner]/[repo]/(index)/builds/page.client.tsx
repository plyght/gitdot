"use client";

import type { BuildResource, RepositoryCommitResource } from "gitdot-api";
import { useState } from "react";
import { BuildRow } from "./ui/build-row";
import { BuildsHeader } from "./ui/builds-header";

export type BuildsFilter = "main" | "pull-request";

export function PageClient({
  owner,
  repo,
  builds,
  commits,
}: {
  owner: string;
  repo: string;
  builds: BuildResource[] | null;
  commits: RepositoryCommitResource[] | null;
}) {
  const [filter, setFilter] = useState<BuildsFilter>("main");
  if (!builds || !commits) return null;
  const commitsBySha: Record<string, RepositoryCommitResource> = {};
  for (const commit of commits) {
    commitsBySha[commit.sha] = commit;
  }
  const filteredBuilds = builds.filter((build) => {
    if (!commitsBySha[build.commit_sha]) return false;
    if (filter === "main") return build.trigger === "push_to_main";
    return build.trigger === "pull_request";
  });

  return (
    <div className="flex flex-col">
      <BuildsHeader
        owner={owner}
        repo={repo}
        filter={filter}
        setFilter={setFilter}
      />
      {filteredBuilds.map((build) => (
        <BuildRow
          key={build.id}
          owner={owner}
          repo={repo}
          build={build}
          commit={commitsBySha[build.commit_sha]}
        />
      ))}
    </div>
  );
}
