import { SpeedInsights } from "@vercel/speed-insights/next";
import type { Metadata } from "next";
import { DatabaseProvider } from "./provider/database";
import { DialogsProvider } from "./provider/dialogs";
import { ShortcutsProvider } from "./provider/shortcuts";
import { ToasterProvider } from "./provider/toaster";
import { UserProvider } from "./provider/user";
import { WorkerProvider } from "./provider/worker";
import { MainFooter } from "./ui/main-footer";

export const metadata: Metadata = {
  title: "gitdot",
  description: "A better open-source GitHub",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <ToasterProvider>
      <DatabaseProvider>
        <WorkerProvider>
          <UserProvider>
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
          </UserProvider>
        </WorkerProvider>
      </DatabaseProvider>
    </ToasterProvider>
  );
}
