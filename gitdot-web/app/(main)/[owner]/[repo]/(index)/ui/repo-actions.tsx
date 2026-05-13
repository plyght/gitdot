"use client";

import type { RepositoryResource } from "gitdot-api";
import { Bell, Download, Star } from "lucide-react";
import { useState } from "react";
import { toast } from "@/(main)/context/toaster";
import { cn } from "@/util";

export function RepoActions({
  repository,
}: {
  repository: RepositoryResource;
}) {
  const [starred, setStarred] = useState(false);
  const [subscribed, setSubscribed] = useState(false);

  const starCount = 142 + (starred ? 1 : 0);
  const subscribeCount = 12 + (subscribed ? 1 : 0);

  const handleClone = () => {
    const url = `${window.location.origin}/${repository.owner}/${repository.name}`;
    navigator.clipboard.writeText(url);
    toast('Copied "git clone url"');
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
        count={starCount}
        active={starred}
        onClick={() => setStarred((v) => !v)}
      />
      <RepoActionButton
        icon={
          <Bell
            className="size-3"
            fill={subscribed ? "currentColor" : "none"}
          />
        }
        label={subscribed ? "Subscribed" : "Subscribe"}
        count={subscribeCount}
        active={subscribed}
        onClick={() => setSubscribed((v) => !v)}
      />
      <RepoActionButton
        icon={<Download className="size-3" />}
        label="Clone"
        onClick={handleClone}
      />
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
          ? "bg-primary text-primary-foreground hover:bg-primary/90"
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
