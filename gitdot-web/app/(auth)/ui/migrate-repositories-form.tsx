"use client";

import type {
  GitHubInstallationResource,
  GitHubRepositoryResource,
  MigrationResource,
} from "gitdot-api";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState, useTransition } from "react";
import {
  getMigrationAction,
  listInstallationRepositoriesAction,
  listInstallationsAction,
  migrateGitHubRepositoriesAction,
} from "@/actions";
import { useTypewriter } from "@/hooks/use-typewriter";
import { cn } from "@/util";
import { timeAgo } from "@/util/date";

export default function MigrateRepositoriesForm({
  username,
  installationId,
}: {
  username: string;
  installationId: number;
}) {
  const [submittedMigration, setSubmittedMigration] =
    useState<MigrationResource | null>(null);

  return (
    <div className="min-h-screen flex items-center justify-center pb-[10vh]">
      <div className="flex flex-col text-sm w-lg">
        {submittedMigration === null ? (
          <NewMigration
            username={username}
            installationId={installationId}
            onSubmitted={setSubmittedMigration}
          />
        ) : (
          <PendingMigration
            username={username}
            migration={submittedMigration}
          />
        )}
      </div>
    </div>
  );
}

function NewMigration({
  username,
  installationId,
  onSubmitted,
}: {
  username: string;
  installationId: number;
  onSubmitted: (migration: MigrationResource) => void;
}) {
  const router = useRouter();
  const [installation, setInstallation] =
    useState<GitHubInstallationResource | null>(null);
  const [repos, setRepos] = useState<GitHubRepositoryResource[] | null>(null);
  const [selectedRepos, setSelectedRepos] = useState<Set<string>>(new Set());
  const [isPending, startTransition] = useTransition();
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    listInstallationsAction().then((list) => {
      if (cancelled) return;
      const match = list.find((i) => i.installation_id === installationId);
      setInstallation(match ?? null);
    });
    return () => {
      cancelled = true;
    };
  }, [installationId]);

  useEffect(() => {
    let cancelled = false;
    setRepos(null);
    listInstallationRepositoriesAction(installationId).then((result) => {
      if (cancelled) return;
      setRepos(result);
    });
    return () => {
      cancelled = true;
    };
  }, [installationId]);

  const sortedRepos = useMemo(() => {
    if (!repos) return repos;
    return [...repos].sort((a, b) => {
      const aTime = a.pushed_at ? new Date(a.pushed_at).getTime() : 0;
      const bTime = b.pushed_at ? new Date(b.pushed_at).getTime() : 0;
      return bTime - aTime;
    });
  }, [repos]);

  const toggleRepo = (fullName: string) => {
    setSelectedRepos((prev) => {
      const next = new Set(prev);
      if (next.has(fullName)) {
        next.delete(fullName);
      } else {
        next.add(fullName);
      }
      return next;
    });
  };

  const handleMigrate = () => {
    if (!installation || !repos) return;
    const repoPayload = Array.from(selectedRepos).flatMap((fullName) => {
      const match = repos.find((r) => r.full_name === fullName);
      return match ? [{ name: match.name, id: match.id }] : [];
    });
    if (repoPayload.length === 0) return;
    setError(null);
    startTransition(async () => {
      const result = await migrateGitHubRepositoriesAction({
        installationId,
        origin: installation.github_login,
        originType: installation.installation_type,
        destination: username,
        destinationType: "user",
        repositories: repoPayload,
        readonly: true,
      });
      if ("error" in result) {
        setError(result.error);
      } else {
        onSubmitted(result.migration);
      }
    });
  };

  return (
    <>
      <div className="flex items-center justify-between pb-2">
        <p>
          {selectedRepos.size === 0
            ? "Import repositories."
            : `Import ${selectedRepos.size} ${
                selectedRepos.size === 1 ? "repository" : "repositories"
              }.`}
        </p>
        {installation && (
          <span className="flex items-center gap-1.5 text-xs text-muted-foreground">
            <Image src="/github-logo.svg" alt="" width={13} height={13} />
            github.com/{installation.github_login}
          </span>
        )}
      </div>
      <div className="flex flex-col h-96 -mx-1 overflow-y-auto scrollbar-thin">
        {sortedRepos === null ? (
          <div className="px-2 py-1.5 font-mono text-muted-foreground">
            loading...
          </div>
        ) : sortedRepos.length === 0 ? (
          <div className="px-2 py-1.5 text-muted-foreground">
            No repositories found.
          </div>
        ) : (
          sortedRepos.map((repo) => {
            const checked = selectedRepos.has(repo.full_name);
            return (
              <button
                key={repo.id}
                type="button"
                onClick={() => toggleRepo(repo.full_name)}
                className="flex items-start gap-2 px-1 py-1.5 text-left hover:bg-accent/50 transition-colors"
              >
                <div
                  className={cn(
                    "mt-[5px] shrink-0 w-3 h-3 rounded-xs border border-border transition-colors duration-150",
                    checked ? "bg-foreground" : "bg-background",
                  )}
                />
                <div className="flex items-end justify-between gap-2 min-w-0 flex-1">
                  <div className="flex flex-col min-w-0">
                    <div className="flex items-center gap-1.5 min-w-0">
                      <span className="truncate">{repo.full_name}</span>
                      {checked && (
                        <span className="text-muted-foreground truncate">
                          → {username}/{repo.name}
                        </span>
                      )}
                    </div>
                    <span
                      className={cn(
                        "truncate text-xs text-muted-foreground",
                        !repo.description && "italic opacity-60",
                      )}
                    >
                      {repo.description ?? "no description"}
                    </span>
                  </div>
                  {repo.pushed_at && (
                    <span className="shrink-0 text-xs font-mono text-muted-foreground">
                      last commit {timeAgo(new Date(repo.pushed_at))}
                    </span>
                  )}
                </div>
              </button>
            );
          })
        )}
      </div>
      <p className="pt-4 text-right">
        <button
          type="button"
          onClick={handleMigrate}
          disabled={isPending || selectedRepos.size === 0 || !installation}
          className="cursor-pointer underline text-foreground decoration-current transition-colors duration-300 disabled:cursor-not-allowed disabled:no-underline disabled:text-muted-foreground"
        >
          {isPending ? "Importing" : "Import"}
        </button>
        <span className="text-muted-foreground mx-1">or</span>
        <button
          type="button"
          onClick={() => router.push(`/${username}`)}
          disabled={isPending}
          className="cursor-pointer underline text-muted-foreground decoration-current hover:text-foreground transition-colors disabled:cursor-not-allowed"
        >
          skip
        </button>
        <span className="text-muted-foreground">.</span>
      </p>
      {error && <p className="pt-2 text-xs text-destructive">{error}</p>}
    </>
  );
}

function PendingMigration({
  username,
  migration,
}: {
  username: string;
  migration: MigrationResource;
}) {
  const router = useRouter();
  const [current, setCurrent] = useState(migration);
  const [fading, setFading] = useState(false);
  const inProgress =
    current.status === "pending" || current.status === "running";

  useEffect(() => {
    if (!inProgress) return;
    let cancelled = false;
    const number = current.number;
    const interval = setInterval(async () => {
      const next = await getMigrationAction(number);
      if (cancelled || !next) return;
      setCurrent(next);
    }, 1000);
    return () => {
      cancelled = true;
      clearInterval(interval);
    };
  }, [current.number, inProgress]);

  useEffect(() => {
    if (inProgress) return;
    const fadeTimer = setTimeout(() => setFading(true), 500);
    const navTimer = setTimeout(() => router.push(`/${username}`), 1200);
    return () => {
      clearTimeout(fadeTimer);
      clearTimeout(navTimer);
    };
  }, [inProgress, router, username]);

  return (
    <div
      className={cn(
        "transition-opacity duration-1000",
        fading ? "opacity-0" : "opacity-100",
      )}
    >
      <div className="flex items-center justify-between pb-2">
        <p>Importing repositories.</p>
        <span className="text-xs">
          <MigrationStatus status={current.status} />
        </span>
      </div>
      <div className="flex flex-col">
        {current.repositories.length === 0 ? (
          <div className="py-1.5 text-muted-foreground">No repositories.</div>
        ) : (
          current.repositories.map((repo) => (
            <div
              key={repo.id}
              className="flex items-center justify-between gap-2 py-0.5"
            >
              <span className="truncate min-w-0">
                {repo.origin_full_name}
                <span className="text-muted-foreground">
                  {" → "}
                  {repo.destination_full_name}
                </span>
              </span>
              <RepositoryStatus status={repo.status} error={repo.error} />
            </div>
          ))
        )}
      </div>
    </div>
  );
}

function MigrationStatus({ status }: { status: string }) {
  switch (status) {
    case "pending":
    case "running":
      return (
        <RunningStatus
          text="importing..."
          className="font-mono text-muted-foreground"
        />
      );
    case "completed":
      return <span className="font-mono text-green-500">completed</span>;
    case "failed":
      return <span className="font-mono text-destructive">failed</span>;
    default:
      return <span className="font-mono text-muted-foreground">{status}</span>;
  }
}

function RunningStatus({
  text,
  className,
}: {
  text: string;
  className?: string;
}) {
  const typed = useTypewriter(text, 60, 1000);
  return (
    <span
      className={cn("inline-block text-left", className)}
      style={{ width: `${text.length}ch` }}
    >
      {typed}
    </span>
  );
}

function RepositoryStatus({
  status,
  error,
}: {
  status: string;
  error: string | null;
}) {
  switch (status) {
    case "pending":
      return (
        <span className="text-xs font-mono text-muted-foreground shrink-0">
          pending
        </span>
      );
    case "running":
      return (
        <span className="text-xs font-mono text-yellow-500 shrink-0">
          running
        </span>
      );
    case "completed":
      return (
        <span className="text-xs font-mono text-green-500 shrink-0">
          completed
        </span>
      );
    case "failed":
      return (
        <span
          className="text-xs font-mono text-destructive shrink-0"
          title={error ?? undefined}
        >
          failed
        </span>
      );
    default:
      return <span className="text-xs font-mono shrink-0">{status}</span>;
  }
}
