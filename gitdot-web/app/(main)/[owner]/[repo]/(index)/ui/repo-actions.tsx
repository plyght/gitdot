"use client";

import type { RepositoryResource } from "gitdot-api";
import { Copy, Download, Settings, Star } from "lucide-react";
import { useState } from "react";
import { toast } from "@/(main)/context/toaster";
import { cn } from "@/util";
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
  const count =
    repository.stars + (starred ? 1 : 0) - (repository.user_star ? 1 : 0);

  const handleClone = () => {
    const url = `${window.location.origin}/${repository.owner}/${repository.name}`;
    navigator.clipboard.writeText(url);
    toast(
      <div className="flex flex-col gap-1">
        <span>Copied to clipboard</span>
        <span className="font-mono bg-accent text-foreground px-1 rounded self-start whitespace-nowrap">
          git clone {url}
        </span>
      </div>,
      {
        icon: <Copy className="size-4" />,
        style: { "--width": "max-content" } as React.CSSProperties,
      },
    );
  };

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
        onClick={handleClone}
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
