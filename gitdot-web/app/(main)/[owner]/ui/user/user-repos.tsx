import type { RepositoryResource } from "gitdot-api";
import Link from "@/ui/link";
import { formatDate } from "@/util";

export function UserRepos({ repos }: { repos: RepositoryResource[] | null }) {
  const publicRepos = repos?.filter((r) => r.visibility === "public") ?? [];
  const privateRepos = repos?.filter((r) => r.visibility === "private") ?? [];

  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-2">
        <span className="text-foreground/40 select-none"># </span>
        Repositories
      </p>
      {repos?.length ? (
        <div className="flex flex-col gap-4">
          {publicRepos.length > 0 && (
            <RepoGroup label="Public" repos={publicRepos} />
          )}
          {privateRepos.length > 0 && (
            <RepoGroup label="Private" repos={privateRepos} />
          )}
        </div>
      ) : (
        <span className="font-mono text-xs">no repos</span>
      )}
    </div>
  );
}

function RepoGroup({
  label,
  repos,
}: {
  label: string;
  repos: RepositoryResource[];
}) {
  const sortedRepos = [...repos].sort(
    (a, b) => Number(!!b.description) - Number(!!a.description),
  );
  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-1.5">
        <span className="text-foreground/40 select-none">## </span>
        {label}
      </p>
      <div className="flex flex-col gap-1">
        {sortedRepos.map((repo) => (
          <RepoRow key={repo.id} repo={repo} />
        ))}
      </div>
    </div>
  );
}

function RepoRow({ repo }: { repo: RepositoryResource }) {
  const seed = parseInt(repo.id.slice(0, 8), 16);
  const commits = (seed % 480) + 3;
  const stars = (seed >>> 4) % 1500;
  const daysAgo = (seed >>> 8) % 90;
  const lastContribution = new Date(Date.now() - daysAgo * 24 * 60 * 60 * 1000);

  return (
    <div className="flex flex-col gap-0.5">
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
          {stars > 0 && (
            <span className="text-xs text-muted-foreground font-mono">
              ({stars})
            </span>
          )}
        </div>
        <span className="text-xs font-mono whitespace-nowrap text-muted-foreground">
          {commits} contributions
          <span className="mx-2">•</span>
          {formatDate(lastContribution)}
        </span>
      </div>
      {repo.description && (
        <span className="text-xs text-foreground truncate">
          {repo.description}
        </span>
      )}
    </div>
  );
}
