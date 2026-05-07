"use client";

import type { RepositoryResource } from "gitdot-api";
import { ChevronDown } from "lucide-react";
import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { formatDate } from "@/util";

type RepoSort = "recent" | "contributions" | "stars";

const REPO_SORT_LABELS: Record<RepoSort, string> = {
  recent: "Recent",
  contributions: "Contributions",
  stars: "Stars",
};

function getRepoStats(repo: RepositoryResource) {
  const seed = parseInt(repo.id.slice(0, 8), 16);
  const commits = (seed % 480) + 3;
  const daysAgo = (seed >>> 8) % 90;
  const lastContribution = new Date(Date.now() - daysAgo * 24 * 60 * 60 * 1000);
  return { commits, daysAgo, lastContribution };
}

export function UserRepos({
  repos,
  isOwner,
}: {
  repos: RepositoryResource[] | null;
  isOwner: boolean;
}) {
  const [sortBy, setSortBy] = useState<RepoSort>("recent");
  const publicRepos = repos?.filter((r) => r.visibility === "public") ?? [];
  const privateRepos = repos?.filter((r) => r.visibility === "private") ?? [];

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
          <DropdownMenuContent align="end" className="min-w-[5rem]">
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
      {repos?.length ? (
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
  repos: RepositoryResource[];
  sortBy: RepoSort;
}) {
  const sortedRepos = [...repos].sort((a, b) => {
    if (sortBy === "stars") return b.stars - a.stars;
    if (sortBy === "contributions")
      return getRepoStats(b).commits - getRepoStats(a).commits;
    return getRepoStats(a).daysAgo - getRepoStats(b).daysAgo;
  });
  return (
    <div>
      {label && (
        <p className="text-xs text-muted-foreground font-mono mb-0.5">
          <span className="text-foreground/40 select-none">## </span>
          {label}
        </p>
      )}
      <div className="flex flex-col gap-1">
        {sortedRepos.map((repo) => (
          <RepoRow key={repo.id} repo={repo} />
        ))}
      </div>
    </div>
  );
}

function RepoRow({ repo }: { repo: RepositoryResource }) {
  const { commits, lastContribution } = getRepoStats(repo);

  return (
    <div className="flex flex-col">
      <div className="flex items-baseline justify-between gap-4">
        <div className="flex items-center gap-1 min-w-0">
          <Link
            href={`/${repo.owner}/${repo.name}`}
            className="text-sm font-medium underline decoration-transparent hover:decoration-current transition-colors duration-200 truncate"
          >
            <span className="font-normal text-muted-foreground">
              {repo.owner}/
            </span>
            {repo.name}
          </Link>
          {repo.stars > 0 && (
            <span className="text-xs text-muted-foreground font-mono">
              ({repo.stars})
            </span>
          )}
        </div>
        <span className="text-xs font-mono whitespace-nowrap text-muted-foreground">
          {commits} contributions
          <span className="mx-1.5">•</span>
          {formatDate(lastContribution)}
        </span>
      </div>
      {repo.description && (
        <div className="text-xs text-foreground truncate pb-1">
          {repo.description}
        </div>
      )}
    </div>
  );
}
