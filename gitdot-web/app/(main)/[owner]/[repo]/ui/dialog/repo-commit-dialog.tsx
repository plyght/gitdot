"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { ExternalLink } from "lucide-react";
import { Suspense, useEffect, useState } from "react";
import { CommitBody } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/commit-body";
import { CommitHeader } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/commit-header";
import { type DiffEntry, renderCommitDiffAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { Loading } from "@/ui/loading";

export type OpenCommitDialogDetail = {
  commit: RepositoryCommitResource;
};

export function RepoCommitDialog({
  owner,
  repo,
}: {
  owner: string;
  repo: string;
}) {
  const [open, setOpen] = useState(false);
  const [commit, setCommit] = useState<RepositoryCommitResource | null>(null);
  const [diffPromise, setDiffPromise] = useState<Promise<DiffEntry[]> | null>(
    null,
  );

  useEffect(() => {
    const handler = (e: Event) => {
      const detail = (e as CustomEvent<OpenCommitDialogDetail>).detail;
      setCommit(detail.commit);
      setDiffPromise(renderCommitDiffAction(owner, repo, detail.commit.sha));
      setOpen(true);
    };
    window.addEventListener("openCommitDialog", handler);
    return () => window.removeEventListener("openCommitDialog", handler);
  }, [owner, repo]);

  if (!commit) return null;
  const shortSha = commit.sha.substring(0, 7);
  const href = `/${owner}/${repo}/commits/${shortSha}`;

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        animations
        className="sm:max-w-none max-w-[70vw] w-[70vw] h-[90vh] max-h-[90vh] p-0 gap-0 flex flex-col"
      >
        <DialogTitle className="sr-only">Commit {shortSha}</DialogTitle>

        <a
          href={href}
          target="_blank"
          rel="noopener noreferrer"
          className="absolute top-2 right-2 z-10 flex items-center gap-1.5 px-2 h-7 text-xs hover:bg-accent rounded text-muted-foreground hover:text-foreground"
        >
          <ExternalLink className="w-3.5 h-3.5" />
          Open in tab
        </a>

        <div className="flex-1 overflow-y-auto scrollbar-thin">
          <div
            data-diff-top
            className="max-w-4xl mx-auto w-full px-4 py-6 flex flex-col gap-6"
          >
            <CommitHeader commit={commit} owner={owner} repo={repo} />
            {diffPromise && (
              <Suspense fallback={<Loading />}>
                <CommitBody diffEntriesPromise={diffPromise} />
              </Suspense>
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
