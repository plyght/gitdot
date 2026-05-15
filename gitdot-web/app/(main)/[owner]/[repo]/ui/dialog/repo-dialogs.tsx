"use client";

import { Suspense } from "react";
import { RepoCommitDialog } from "./repo-commit-dialog";
import { RepoFileDialog } from "./repo-file-dialog";

export function RepoDialogs({ owner, repo }: { owner: string; repo: string }) {
  return (
    <Suspense fallback={null}>
      <RepoFileDialog owner={owner} repo={repo} />
      <RepoCommitDialog owner={owner} repo={repo} />
    </Suspense>
  );
}
