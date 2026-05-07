import type { RepositoryResource } from "gitdot-api";
import Link from "@/ui/link";
import { formatDate } from "@/util";

export function UserRepos({ repos }: { repos: RepositoryResource[] | null }) {
  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-2">
        <span className="text-foreground/40 select-none"># </span>
        Repositories
      </p>
      {repos?.length ? (
        <div className="flex flex-col gap-3">
          {repos.map((repo) => {
            const seed = parseInt(repo.id.slice(0, 8), 16);
            const commits = (seed % 480) + 3;
            const stars = (seed >>> 4) % 1500;
            const daysAgo = (seed >>> 8) % 90;
            const lastContribution = new Date(
              Date.now() - daysAgo * 24 * 60 * 60 * 1000,
            );
            return (
              <div
                key={repo.id}
                className="flex items-start justify-between gap-4"
              >
                <div className="flex flex-col gap-0.5">
                  <Link
                    href={`/${repo.owner}/${repo.name}`}
                    className="text-sm font-medium underline decoration-transparent hover:decoration-current transition-colors duration-200"
                  >
                    <span className="font-normal text-muted-foreground">
                      {repo.owner}/
                    </span>
                    {repo.name}
                  </Link>
                  <span className="text-xs text-muted-foreground font-mono">
                    {stars} stars · {repo.visibility}
                  </span>
                </div>
                <div className="flex flex-col gap-0.5 items-end">
                  <span className="text-xs font-mono">{commits} commits</span>
                  <span className="text-xs text-muted-foreground font-mono">
                    last commit {formatDate(lastContribution)}
                  </span>
                </div>
              </div>
            );
          })}
        </div>
      ) : (
        <span className="font-mono text-xs">no repos</span>
      )}
    </div>
  );
}
