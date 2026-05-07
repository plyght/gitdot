"use client";

import { NewRepoDialog } from "@/(main)/ui/new-repo-dialog";

export function NewRepoProvider({ children }: { children: React.ReactNode }) {
  return (
    <>
      {children}
      <NewRepoDialog />
    </>
  );
}
