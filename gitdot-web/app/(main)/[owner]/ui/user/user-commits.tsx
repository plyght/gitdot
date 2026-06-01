"use client";

import type { UserCommitResource } from "gitdot-api";
import { useState } from "react";
import { useTimezone } from "@/(main)/context/timezone";
import { formatDateIso, subtractMonths } from "@/util/date";
import { UserCommitStatistics } from "./user-commit-statistics";
import { UserCommitsCalendar } from "./user-commits-calendar";
import { UserCommitsHeader } from "./user-commits-header";
import { UserCommitsLog } from "./user-commits-log";

export function UserCommits({ commits }: { commits: UserCommitResource[] }) {
  const tz = useTimezone();

  const [startDate, setStartDate] = useState(() =>
    subtractMonths(new Date(), 11, tz),
  );
  const [endDate, setEndDate] = useState(() => formatDateIso(new Date(), tz));
  const [selectedMonth, setSelectedMonth] = useState<string | null>(null);

  const commitMap = new Map<string, UserCommitResource[]>();
  for (const c of commits) {
    const day = formatDateIso(new Date(c.date), tz);
    if (!commitMap.has(day)) commitMap.set(day, []);
    commitMap.get(day)?.push(c);
  }

  const years = getCommitYears(commitMap);

  return (
    <div className="flex flex-col h-full min-h-0">
      <UserCommitsHeader
        years={years}
        endDate={endDate}
        setStartDate={setStartDate}
        setEndDate={setEndDate}
        setSelectedMonth={setSelectedMonth}
      />
      <UserCommitsCalendar
        commits={commitMap}
        startDate={startDate}
        endDate={endDate}
        selectedMonth={selectedMonth}
        setSelectedMonth={setSelectedMonth}
      />
      <div className="flex-1 min-h-0 overflow-y-auto scrollbar-none">
        <UserCommitStatistics
          commits={commitMap}
          startDate={startDate}
          endDate={endDate}
          selectedMonth={selectedMonth}
        />
        <UserCommitsLog
          commits={commitMap}
          startDate={startDate}
          endDate={endDate}
          selectedMonth={selectedMonth}
        />
      </div>
    </div>
  );
}

// Years span from the user's earliest commit through the current year, filling
// any gaps so the dropdown stays consecutive (newest first).
function getCommitYears(
  commitMap: Map<string, UserCommitResource[]>,
): number[] {
  const currentYear = new Date().getFullYear();
  let earliestYear = currentYear;
  for (const day of commitMap.keys()) {
    const year = Number(day.slice(0, 4));
    if (year < earliestYear) earliestYear = year;
  }
  const years: number[] = [];
  for (let y = currentYear; y >= earliestYear; y--) years.push(y);
  return years;
}
