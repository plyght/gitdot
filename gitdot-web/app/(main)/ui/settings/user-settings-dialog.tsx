"use client";

import type { UserResource } from "gitdot-api";
import { useEffect, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { useUserContext } from "@/(main)/context/user";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { UserSettingsAccount } from "./user-settings-account";
import { UserSettingsAppearance } from "./user-settings-appearance";
import { UserSettingsInstallations } from "./user-settings-installations";
import { UserSettingsMigrations } from "./user-settings-migrations";
import { UserSettingsProfile } from "./user-settings-profile";
import {
  UserSettingsSidebar,
  type UserSettingsTab,
} from "./user-settings-sidebar";

export function UserSettingsDialog() {
  const { user } = useUserContext();
  if (!user) return null;

  return <UserSettingsDialogInner user={user} />;
}

function UserSettingsDialogInner({ user }: { user: UserResource }) {
  const [open, setOpen] = useState(false);
  const [tab, setTab] = useState<UserSettingsTab>("profile");

  useShortcuts([
    {
      name: "Settings",
      description: "Open settings",
      keys: [","],
      execute: () => setOpen(true),
    },
  ]);

  useEffect(() => {
    const handle = () => setOpen(true);
    window.addEventListener("openUserSettings", handle);
    return () => window.removeEventListener("openUserSettings", handle);
  }, []);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-[60vw]! h-[80vh]! p-0! gap-0! overflow-hidden flex flex-col"
        animations={true}
        showOverlay={true}
        aria-describedby={undefined}
      >
        <DialogTitle className="sr-only">User Settings</DialogTitle>

        <div className="flex flex-1 min-h-0 overflow-hidden font-mono text-sm">
          <UserSettingsSidebar tab={tab} onTabChange={setTab} />

          <div className="flex-1 overflow-y-auto scrollbar-thin">
            {tab === "profile" && <UserSettingsProfile user={user} />}
            {tab === "account" && (
              <UserSettingsAccount setUserSettingsOpen={setOpen} />
            )}
            {tab === "appearance" && <UserSettingsAppearance />}
            {tab === "installations" && <UserSettingsInstallations />}
            {tab === "migrations" && <UserSettingsMigrations />}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
