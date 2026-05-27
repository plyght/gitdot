"use client";

import { type ResourceResultType, useResources } from "gitdot-dal/client";
import { Suspense } from "react";
import type { Resources } from "../../layout";
import { RepoCommitDialog } from "./repo-commit-dialog";
import { RepoFileDialog } from "./repo-file-dialog";

export function RepoDialogs({
  owner,
  repo,
  resources,
}: {
  owner: string;
  repo: string;
  resources: ResourceResultType<Resources>;
}) {
  const promises = useResources(resources);

  return (
    <>
      <Suspense fallback={null}>
        <RepoFileDialog
          owner={owner}
          repo={repo}
          pathsPromise={promises.paths}
        />
      </Suspense>
      <Suspense fallback={null}>
        <RepoCommitDialog owner={owner} repo={repo} />
      </Suspense>
    </>
  );
}
