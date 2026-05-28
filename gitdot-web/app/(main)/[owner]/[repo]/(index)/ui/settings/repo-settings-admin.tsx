"use client";

import type { RepositoryResource } from "gitdot-api";
import { useState } from "react";
import { cn } from "@/util";
import { RepoDeleteDialog } from "./repo-delete-dialog";
import { RepoPromoteDialog } from "./repo-promote-dialog";

export function RepoSettingsAdmin({
  repository,
}: {
  repository: RepositoryResource;
}) {
  const [deleteOpen, setDeleteOpen] = useState(false);
  const [promoteOpen, setPromoteOpen] = useState(false);

  return (
    <>
      <div className="divide-y divide-border">
        {repository.readonly && (
          <Action
            title="Promote repository"
            description="Collaborators will be able to push directly to gitdot and new commits on GitHub will no longer be synced."
            actionLabel="Promote"
            onAction={() => setPromoteOpen(true)}
          />
        )}
        <Action
          title="Delete repository"
          description="Permanently remove this repository and all of its data. This cannot be undone."
          actionLabel="Delete"
          destructive
          onAction={() => setDeleteOpen(true)}
        />
      </div>
      <RepoDeleteDialog
        open={deleteOpen}
        setOpen={setDeleteOpen}
        owner={repository.owner}
        repo={repository.name}
      />
      <RepoPromoteDialog
        open={promoteOpen}
        setOpen={setPromoteOpen}
        owner={repository.owner}
        repo={repository.name}
      />
    </>
  );
}

function Action({
  title,
  description,
  actionLabel,
  destructive = false,
  onAction,
}: {
  title: string;
  description: string;
  actionLabel: string;
  destructive?: boolean;
  onAction: () => void;
}) {
  return (
    <div className="p-3">
      <p className="text-sm font-medium dark:font-normal">{title}</p>
      <p className="text-sm text-muted-foreground">{description}</p>
      <div className="flex justify-start mt-3">
        <button
          type="button"
          onClick={onAction}
          className={cn(
            "text-sm underline underline-offset-2 cursor-pointer transition-colors",
            destructive
              ? "text-destructive hover:text-destructive/80"
              : "text-muted-foreground hover:text-foreground",
          )}
        >
          {actionLabel}
        </button>
      </div>
    </div>
  );
}
