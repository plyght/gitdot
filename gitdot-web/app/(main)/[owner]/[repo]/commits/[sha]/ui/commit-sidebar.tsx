"use client";

import type { CommitDiffResource } from "gitdot-api";
import { cn } from "@/util";

export function CommitSidebar({
  diffs,
  selectedIndex,
  onSelect,
}: {
  diffs: CommitDiffResource[];
  selectedIndex: number;
  onSelect: (index: number) => void;
}) {
  if (diffs.length === 0) return null;

  return (
    <div className="min-w-0">
      <div className="text-xs font-mono text-foreground mb-1">Files</div>
      <nav className="flex flex-col gap-1 font-mono text-xs">
        {diffs.map((entry, i) => {
          const path = entry.path;
          const name = path.split("/").slice(-2).join("/");
          const isCurrent = selectedIndex === i;
          return (
            <button
              key={path}
              type="button"
              title={path}
              onClick={() => {
                onSelect(i);
                document.getElementById(path)?.scrollIntoView();
              }}
              className={cn(
                "text-left truncate underline cursor-pointer",
                isCurrent
                  ? "text-foreground decoration-current"
                  : "text-muted-foreground decoration-transparent hover:text-foreground hover:decoration-current",
              )}
            >
              {name}
            </button>
          );
        })}
      </nav>
    </div>
  );
}
