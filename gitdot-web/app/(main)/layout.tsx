import { SpeedInsights } from "@vercel/speed-insights/next";
import type { Metadata } from "next";
import { headers } from "next/headers";
import { CommitDialog } from "@/(main)/ui/commit-dialog";
import { ImportRepoDialog } from "@/(main)/ui/import-repo-dialog";
import { InstallCliDialog } from "@/(main)/ui/install-cli-dialog";
import { NewOrgDialog } from "@/(main)/ui/new-org-dialog";
import { NewRepoDialog } from "@/(main)/ui/new-repo-dialog";
import { RepoSwitcherDialog } from "@/(main)/ui/repo-switcher-dialog";
import { UserSettingsDialog } from "@/(main)/ui/settings/user-settings-dialog";
import { DalProvider } from "./context/dal-provider";
import { ShortcutsProvider } from "./context/shortcuts";
import { TimezoneProvider } from "./context/timezone";
import { ToasterProvider } from "./context/toaster";
import { UserProvider } from "./context/user";
import { MainFooter } from "./ui/main-footer";

export const metadata: Metadata = {
  title: "gitdot",
  description: "A better open-source GitHub",
};

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const timezone =
    (await headers()).get("x-vercel-ip-timezone") ??
    Intl.DateTimeFormat().resolvedOptions().timeZone;

  return (
    <ToasterProvider>
      <DalProvider>
        <UserProvider>
          <TimezoneProvider timezone={timezone}>
            <ShortcutsProvider>
              <div className="flex flex-col h-screen w-full max-w-screen overflow-hidden">
                <main className="flex-1 min-h-0 overflow-hidden">
                  {children}
                </main>
                <MainFooter />
                <SpeedInsights />
                <RepoSwitcherDialog />
                <NewOrgDialog />
                <NewRepoDialog />
                <ImportRepoDialog />
                <UserSettingsDialog />
                <CommitDialog />
                <InstallCliDialog />
              </div>
            </ShortcutsProvider>
          </TimezoneProvider>
        </UserProvider>
      </DalProvider>
    </ToasterProvider>
  );
}
