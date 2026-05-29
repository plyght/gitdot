import type { DiffEntry } from "gitdot-dal/client";
import { CommitDiffHeader } from "./commit-diff-header";
import { DiffBody } from "./diff-body";

export function CommitDiffFile({ entry }: { entry: DiffEntry }) {
  return (
    <div
      data-diff-file
      className="rounded-sm border border-border overflow-hidden scroll-mt-4"
    >
      <CommitDiffHeader
        path={entry.path}
        linesAdded={entry.linesAdded}
        linesRemoved={entry.linesRemoved}
      />
      <DiffBody spans={entry.spans} />
    </div>
  );
}
