import type { DiffData } from "gitdot-dal/client";
import { CommitDiffFile } from "./commit-diff-file";

export function CommitBody({ entries }: { entries: DiffData }) {
  return (
    <div className="flex flex-col gap-4">
      {entries.map((entry) => (
        <CommitDiffFile key={entry.path} entry={entry} />
      ))}
    </div>
  );
}
