import { SpeedInsights } from "@vercel/speed-insights/next";
import type { Metadata } from "next";
import { headers } from "next/headers";
import { DatabaseProvider } from "./provider/database";
import { DialogsProvider } from "./provider/dialogs";
import { ShortcutsProvider } from "./provider/shortcuts";
import { TimezoneProvider } from "./provider/timezone";
import { ToasterProvider } from "./provider/toaster";
import { UserProvider } from "./provider/user";
import { WorkerProvider } from "./provider/worker";
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
      <DatabaseProvider>
        <WorkerProvider>
          <UserProvider>
            <TimezoneProvider timezone={timezone}>
              <ShortcutsProvider>
                <DialogsProvider>
                  <div className="flex flex-col h-screen w-full max-w-screen overflow-hidden">
                    <main className="flex-1 min-h-0 overflow-hidden">
                      {children}
                    </main>
                    <MainFooter />
                    <SpeedInsights />
                  </div>
                </DialogsProvider>
              </ShortcutsProvider>
            </TimezoneProvider>
          </UserProvider>
        </WorkerProvider>
      </DatabaseProvider>
    </ToasterProvider>
  );
}
