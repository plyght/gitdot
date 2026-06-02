"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { RepositoryResource } from "gitdot-api";
import { Copy } from "lucide-react";
import { toast } from "@/(main)/context/toaster";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function RepoCloneDialog({
  repository,
  open,
  onOpenChange,
}: {
  repository: RepositoryResource;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const url =
    typeof window !== "undefined"
      ? `${window.location.origin}/${repository.owner}/${repository.name}`
      : `/${repository.owner}/${repository.name}`;
  const command = `git clone ${url}`;

  const handleCopy = () => {
    navigator.clipboard.writeText(command);
    toast.success("Copied to clipboard");
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        className="p-0 gap-0 overflow-hidden w-[28rem] top-[45%]"
        animations
        showOverlay
      >
        <VisuallyHidden>
          <DialogTitle>Clone repository</DialogTitle>
        </VisuallyHidden>
        <button
          type="button"
          onClick={handleCopy}
          aria-label="Copy clone command"
          className="group flex items-center w-full border-b border-border text-left cursor-pointer"
        >
          <span className="flex-1 min-w-0 px-2 py-2 font-mono text-sm whitespace-nowrap overflow-x-auto">
            {command}
          </span>
          <span className="flex items-center justify-center px-2 self-stretch text-muted-foreground group-hover:text-foreground transition-colors">
            <Copy className="size-3.5" />
          </span>
        </button>
        <div className="flex items-center justify-between h-7">
          <span className="px-2 text-xs text-muted-foreground truncate">
            Install <span className="font-mono">gitdot-cli</span> to push and
            clone private repos
          </span>
          <button
            type="button"
            onClick={() =>
              window.dispatchEvent(new CustomEvent("openInstallCli"))
            }
            className="flex items-center px-3 h-full text-xs bg-primary text-primary-foreground border-l border-primary hover:bg-primary/80 transition-colors cursor-pointer"
          >
            Install
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
