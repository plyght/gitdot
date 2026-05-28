"use client";

import type { RepositoryResource } from "gitdot-api";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { RepoSettingsAdmin } from "./repo-settings-admin";
import { RepoSettingsInfo } from "./repo-settings-info";
import {
  RepoSettingsSidebar,
  type RepoSettingsTab,
} from "./repo-settings-sidebar";

export function RepoSettingsDialog({
  repository,
  open,
  onOpenChange,
  tab,
  onTabChange,
}: {
  repository: RepositoryResource;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  tab: RepoSettingsTab;
  onTabChange: (tab: RepoSettingsTab) => void;
}) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        className="max-w-[60vw]! h-[80vh]! p-0! gap-0! overflow-hidden flex flex-col"
        animations={true}
        showOverlay={true}
        aria-describedby={undefined}
      >
        <DialogTitle className="sr-only">Repository settings</DialogTitle>

        <div className="flex flex-1 min-h-0 overflow-hidden font-mono text-sm">
          <RepoSettingsSidebar tab={tab} onTabChange={onTabChange} />

          <div className="flex-1 overflow-y-auto scrollbar-thin">
            {tab === "info" && <RepoSettingsInfo repository={repository} />}
            {tab === "admin" && <RepoSettingsAdmin repository={repository} />}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
