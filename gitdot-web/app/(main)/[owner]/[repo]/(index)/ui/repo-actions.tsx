"use client";

import type { RepositoryResource } from "gitdot-api";
import { Download, Settings, Star } from "lucide-react";
import { useState } from "react";
import { cn } from "@/util";
import { RepoCloneDialog } from "./repo-clone-dialog";
import { RepoSettingsDialog } from "./settings/repo-settings-dialog";
import type { RepoSettingsTab } from "./settings/repo-settings-sidebar";

export function RepoActions({
  repository,
  starred,
  toggleStar,
  isAdmin,
}: {
  repository: RepositoryResource;
  starred: boolean;
  toggleStar: () => void;
  isAdmin: boolean;
}) {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [settingsTab, setSettingsTab] = useState<RepoSettingsTab>("info");
  const [cloneOpen, setCloneOpen] = useState(false);
  const count =
    repository.stars + (starred ? 1 : 0) - (repository.user_star ? 1 : 0);

  return (
    <div className="flex flex-col py-2 border-b">
      <span className="flex items-center gap-1.5 text-xs text-muted-foreground font-mono px-2 mb-1">
        Actions
      </span>
      <RepoActionButton
        icon={
          <Star className="size-3" fill={starred ? "currentColor" : "none"} />
        }
        label={starred ? "Starred" : "Star"}
        count={count}
        active={starred}
        onClick={toggleStar}
      />
      <RepoActionButton
        icon={<Download className="size-3" />}
        label="Clone"
        onClick={() => setCloneOpen(true)}
      />
      <RepoCloneDialog
        repository={repository}
        open={cloneOpen}
        onOpenChange={setCloneOpen}
      />
      {isAdmin && (
        <>
          <RepoActionButton
            icon={<Settings className="size-3" />}
            label="Settings"
            onClick={() => {
              setSettingsTab("info");
              setSettingsOpen(true);
            }}
          />
          <RepoSettingsDialog
            repository={repository}
            open={settingsOpen}
            onOpenChange={setSettingsOpen}
            tab={settingsTab}
            onTabChange={setSettingsTab}
          />
        </>
      )}
    </div>
  );
}

function RepoActionButton({
  icon,
  label,
  count,
  active,
  primary,
  onClick,
  className,
}: {
  icon: React.ReactNode;
  label: string;
  count?: number;
  active?: boolean;
  primary?: boolean;
  onClick: () => void;
  className?: string;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        "w-full h-6 flex items-center justify-start gap-1.5 px-2 text-xs font-mono cursor-default transition-colors focus:outline-none",
        primary
          ? "bg-highlight text-highlight-foreground hover:bg-highlight/90"
          : active
            ? "bg-accent text-foreground"
            : "hover:bg-accent text-foreground",
        className,
      )}
    >
      {icon}
      <span>{label}</span>
      {count !== undefined && (
        <span className="ml-auto tabular-nums">{count}</span>
      )}
    </button>
  );
}
