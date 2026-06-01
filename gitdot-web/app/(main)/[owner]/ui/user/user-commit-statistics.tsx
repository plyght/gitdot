"use client";

import type { UserCommitResource } from "gitdot-api";
import { dateInRange } from "@/util/date";

export function UserCommitStatistics({
  commits,
  view,
  startDate,
  endDate,
  selectedMonth,
}: {
  commits: Map<string, UserCommitResource[]>;
  view: "recent" | number;
  startDate: string;
  endDate: string;
  selectedMonth: string | null;
}) {
  const visibleCommits = [...commits.entries()]
    .filter(
      ([date]) =>
        dateInRange(date, startDate, endDate) &&
        (selectedMonth === null || date.startsWith(selectedMonth)),
    )
    .flatMap(([, cs]) => cs);

  const repoCounts = new Map<string, number>();
  let redactedCount = 0;
  for (const c of visibleCommits) {
    if (c.redacted || !c.repo_name) {
      redactedCount++;
      continue;
    }
    repoCounts.set(c.repo_name, (repoCounts.get(c.repo_name) ?? 0) + 1);
  }
  const repoList = [...repoCounts.entries()].sort((a, b) => b[1] - a[1]);
  const totalCommits = visibleCommits.length;

  const label = selectedMonth
    ? new Date(`${selectedMonth}-01T00:00:00`).toLocaleString("en-US", {
        month: "long",
      })
    : view === "recent"
      ? "Recent"
      : String(view);

  const parts = repoList.map(([r, c]) => `${r} (${c})`);
  if (redactedCount > 0) parts.push(`private (${redactedCount})`);
  const sentence = `${totalCommits} commits to ${parts.join(", ")}`;

  return (
    <div className="mt-2 px-3">
      <p className="text-xs text-muted-foreground font-mono mb-1">{label}</p>
      <p className="text-xs font-mono">{sentence}</p>
    </div>
  );
}
