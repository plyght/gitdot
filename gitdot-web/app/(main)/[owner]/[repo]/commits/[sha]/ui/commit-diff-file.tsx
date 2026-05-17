import type { DiffEntry } from "@/actions";
import { CommitDiffHeader } from "./commit-diff-header";
import { DiffBody } from "./diff-body";

export function CommitDiffFile({ entry }: { entry: DiffEntry }) {
  return (
    <div
      data-diff-file
      className="rounded-sm border border-border overflow-hidden scroll-mt-4"
    >
      <CommitDiffHeader
        path={entry.resource.path}
        linesAdded={entry.resource.lines_added}
        linesRemoved={entry.resource.lines_removed}
      />
      <DiffBody spans={entry.spans} />
    </div>
  );
}
