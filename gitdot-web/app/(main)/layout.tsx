import { SpeedInsights } from "@vercel/speed-insights/next";
import type { Metadata } from "next";
import { headers } from "next/headers";
import { cloneElement, type ReactElement, type ReactNode } from "react";
import { CommitDialog } from "@/(main)/ui/commit-dialog";
import { MigrateRepoDialog } from "@/(main)/ui/migrate-repo-dialog";
import { NewOrgDialog } from "@/(main)/ui/new-org-dialog";
import { NewRepoDialog } from "@/(main)/ui/new-repo-dialog";
import { RepoSwitcherDialog } from "@/(main)/ui/repo-switcher-dialog";
import { SettingsDialog } from "@/(main)/ui/settings/settings-dialog";
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

function withProviders(providers: ReactElement[], children: ReactNode) {
  return providers.reduceRight(
    (acc, provider) => cloneElement(provider, undefined, acc),
    children,
  );
}

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const timezone =
    (await headers()).get("x-vercel-ip-timezone") ??
    Intl.DateTimeFormat().resolvedOptions().timeZone;

  const providers = [
    <ToasterProvider key="toaster" />,
    <DalProvider key="dal" />,
    <UserProvider key="user" />,
    <TimezoneProvider key="timezone" timezone={timezone} />,
    <ShortcutsProvider key="shortcuts" />,
  ];

  const dialogs = [
    <RepoSwitcherDialog key="repo-switcher" />,
    <NewOrgDialog key="new-org" />,
    <NewRepoDialog key="new-repo" />,
    <MigrateRepoDialog key="migrate-repo" />,
    <SettingsDialog key="settings" />,
    <CommitDialog key="commit" />,
  ];

  return withProviders(
    providers,
    <div className="flex flex-col h-screen w-full max-w-screen overflow-hidden">
      <main className="flex-1 min-h-0 overflow-hidden">{children}</main>
      <MainFooter />
      <SpeedInsights />
      {dialogs}
    </div>,
  );
}
