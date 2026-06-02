"use client";

import type { UserRepositoryResource } from "gitdot-api";
import { ChevronDown } from "lucide-react";
import { useMemo, useState } from "react";
import { useTimezone } from "@/(main)/context/timezone";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { formatDate } from "@/util/date";

type RepoSort = "recent" | "contributions" | "stars";

const REPO_SORT_LABELS: Record<RepoSort, string> = {
  recent: "Recent",
  contributions: "Contributions",
  stars: "Stars",
};

type Repository = {
  owner: string;
  name: string;
  description?: string;
  stars: number;
  visibility: string;
  count: number;
  lastDate: Date | null;
};

export function UserRepos({
  repos,
  contributed,
  isOwner,
}: {
  repos: UserRepositoryResource[];
  contributed: UserRepositoryResource[];
  isOwner: boolean;
}) {
  const [sortBy, setSortBy] = useState<RepoSort>("recent");
  const repositories = useMemo(
    () => buildRepositories(repos, contributed),
    [repos, contributed],
  );
  const publicRepos = repositories.filter((r) => r.visibility === "public");
  const privateRepos = repositories.filter((r) => r.visibility === "private");

  return (
    <div>
      <div className="flex items-baseline justify-between mb-2">
        <span className="text-xs text-muted-foreground font-mono">
          <span className="text-foreground/40 select-none"># </span>
          Repositories
        </span>
        <DropdownMenu>
          <DropdownMenuTrigger className="flex items-center gap-0.5 text-xs text-muted-foreground/60 font-mono cursor-pointer transition-colors hover:text-foreground">
            {REPO_SORT_LABELS[sortBy]}
            <ChevronDown className="size-3" />
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" className="min-w-20">
            {(Object.keys(REPO_SORT_LABELS) as RepoSort[]).map((key) => (
              <DropdownMenuItem
                key={key}
                className="text-xs font-mono"
                onClick={() => setSortBy(key)}
              >
                {REPO_SORT_LABELS[key]}
              </DropdownMenuItem>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      {repositories.length ? (
        isOwner ? (
          <div className="flex flex-col gap-4">
            {publicRepos.length > 0 && (
              <RepoGroup label="Public" repos={publicRepos} sortBy={sortBy} />
            )}
            {privateRepos.length > 0 && (
              <RepoGroup label="Private" repos={privateRepos} sortBy={sortBy} />
            )}
          </div>
        ) : (
          <RepoGroup repos={publicRepos} sortBy={sortBy} />
        )
      ) : (
        <span className="font-mono text-xs">no repos</span>
      )}
    </div>
  );
}

function RepoGroup({
  label,
  repos,
  sortBy,
}: {
  label?: string;
  repos: Repository[];
  sortBy: RepoSort;
}) {
  const sortedRepos = [...repos].sort((a, b) => {
    if (sortBy === "stars") return b.stars - a.stars;
    if (sortBy === "contributions") return b.count - a.count;
    // Date-less repos sort last; 0 (not -Infinity) avoids a NaN comparison
    // between two such repos, which would order them nondeterministically
    // across JS engines and break SSR hydration.
    const ta = a.lastDate ? a.lastDate.getTime() : 0;
    const tb = b.lastDate ? b.lastDate.getTime() : 0;
    return tb - ta;
  });
  return (
    <div>
      {label && (
        <p className="text-xs text-muted-foreground font-mono mb-0.5">
          <span className="text-foreground/40 select-none">## </span>
          {label}
        </p>
      )}
      <div className="flex flex-col">
        {sortedRepos.map((repo) => (
          <RepoRow key={`${repo.owner}/${repo.name}`} repo={repo} />
        ))}
      </div>
    </div>
  );
}

function RepoRow({ repo }: { repo: Repository }) {
  const tz = useTimezone();
  return (
    <Link
      href={`/${repo.owner}/${repo.name}`}
      className="group flex flex-col w-full py-0.5 cursor-pointer outline-none"
    >
      <div className="flex items-baseline justify-between gap-4">
        <div className="flex items-center gap-1 min-w-0">
          <span className="text-sm font-medium dark:font-normal underline decoration-transparent group-hover:decoration-current group-focus:decoration-current transition-colors duration-200 truncate">
            <span className="font-normal text-muted-foreground">
              {repo.owner}/
            </span>
            {repo.name}
          </span>
          {repo.stars > 0 && (
            <span className="text-xs text-muted-foreground font-mono">
              ({repo.stars})
            </span>
          )}
        </div>
        <span className="text-xs font-mono whitespace-nowrap text-muted-foreground">
          {repo.lastDate ? (
            <>
              {repo.count} contributions
              <span className="mx-1.5">•</span>
              {formatDate(repo.lastDate, tz)}
            </>
          ) : (
            "no contributions"
          )}
        </span>
      </div>
      {repo.description && (
        <div className="text-xs text-foreground truncate pb-1">
          {repo.description}
        </div>
      )}
    </Link>
  );
}

function buildRepositories(
  repos: UserRepositoryResource[],
  contributed: UserRepositoryResource[],
): Repository[] {
  const repositories: Repository[] = [];
  const seen = new Set<string>();

  // TODO: owned repos carry the user's own all-time commit stats
  // confusing to show recently contributed repos mixed with owned repos
  for (const r of repos) {
    seen.add(`${r.owner}/${r.name}`);
    repositories.push({
      owner: r.owner,
      name: r.name,
      description: r.description,
      stars: r.stars,
      visibility: r.visibility,
      count: r.commit_count ?? 0,
      lastDate: r.last_commit_at ? new Date(r.last_commit_at) : null,
    });
  }

  for (const c of contributed) {
    const key = `${c.owner}/${c.name}`;
    if (seen.has(key)) continue;
    repositories.push({
      owner: c.owner,
      name: c.name,
      description: c.description,
      stars: c.stars,
      visibility: c.visibility,
      count: c.commit_count ?? 0,
      lastDate: c.last_commit_at ? new Date(c.last_commit_at) : null,
    });
  }

  return repositories;
}
