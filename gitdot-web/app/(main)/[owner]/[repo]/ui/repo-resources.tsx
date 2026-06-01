"use client";

import { ClientProvider } from "gitdot-dal/client";
import { useEffect } from "react";

export function RepoResources({
  owner,
  repo,
  children,
}: {
  owner: string;
  repo: string;
  children: React.ReactNode;
}) {
  useEffect(() => {
    ClientProvider.instance.syncRepo(owner, repo);
  }, [owner, repo]);

  return <>{children}</>;
}
