import { use } from "react";
import type { DiffEntry } from "@/actions";
import { CommitDiffFile } from "./commit-diff-file";

export function CommitBody({
  diffEntriesPromise,
}: {
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const entries = use(diffEntriesPromise);

  return (
    <div className="flex flex-col gap-4">
      {entries.map((entry) => (
        <CommitDiffFile key={entry.resource.path} entry={entry} />
      ))}
    </div>
  );
}
