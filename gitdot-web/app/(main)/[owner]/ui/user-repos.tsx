import type { RepositoryResource } from "gitdot-api";
import Link from "@/ui/link";
import { formatDate } from "@/util";

export function UserRepos({
  owner,
  repos,
}: {
  owner: string;
  repos: RepositoryResource[] | null;
}) {
  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-2">
        <span className="text-foreground/40 select-none"># </span>
        repos
      </p>
      {repos?.length ? (
        <div className="flex flex-col gap-1">
          {repos.map((repo) => {
            const stars = parseInt(repo.id.slice(0, 8), 16) % 1000;
            return (
              <div
                key={repo.id}
                className="flex items-baseline justify-between gap-4"
              >
                <div className="flex items-baseline gap-1">
                  <Link
                    href={`/${owner}/${repo.name}`}
                    className="text-sm font-medium underline decoration-transparent hover:decoration-current transition-colors duration-200"
                  >
                    {repo.name}
                  </Link>
                  <span className="text-xs text-muted-foreground font-mono">
                    ({stars})
                  </span>
                </div>
                <span className="text-xs text-muted-foreground font-mono">
                  {repo.visibility} · {formatDate(new Date(repo.created_at))}
                </span>
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
