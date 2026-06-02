"use client";

import { useRouter } from "next/navigation";
import { useEffect, useState, useTransition } from "react";
import { getGithubAppInstallUrlAction } from "@/actions";

export default function InstallGithubAppForm({
  username,
}: {
  username: string;
}) {
  const [visible, setVisible] = useState(false);
  const [isInstalling, startInstall] = useTransition();
  const router = useRouter();

  useEffect(() => {
    const t = setTimeout(() => setVisible(true), 50);
    return () => clearTimeout(t);
  }, []);

  function handleInstall() {
    startInstall(async () => {
      const result = await getGithubAppInstallUrlAction("onboarding");
      if ("url" in result) {
        window.location.href = result.url;
      }
    });
  }

  return (
    <div className="min-h-screen flex items-center justify-center pb-[10vh]">
      <div
        className="flex flex-col text-sm w-lg transition-opacity duration-1000"
        style={{ opacity: visible ? 1 : 0 }}
      >
        <p>Import repositories.</p>
        <p className="text-muted-foreground leading-relaxed">
          Mirror your GitHub repositories on gitdot.
        </p>
        <p className="pt-3 text-muted-foreground leading-relaxed">
          Imported repositories are <b>read-only mirrors.</b>
        </p>
        <p className="text-muted-foreground leading-relaxed">
          New commits pushed to GitHub are replicated to gitdot automatically
          and you <b>cannot</b> push directly to the gitdot mirror.
        </p>
        <p className="pt-2 pb-4 text-muted-foreground leading-relaxed">
          Note: you can promote a mirror to a full gitdot repository at any time
        </p>
        <p className="text-right">
          <button
            type="button"
            onClick={handleInstall}
            disabled={isInstalling}
            className="cursor-pointer underline text-foreground decoration-current transition-colors disabled:cursor-not-allowed"
          >
            Import
          </button>
          <span className="text-muted-foreground mx-1">or</span>
          <button
            type="button"
            onClick={() => router.push(`/${username}`)}
            className="cursor-pointer underline text-muted-foreground decoration-current hover:text-foreground transition-colors"
          >
            skip
          </button>
          <span className="text-muted-foreground">.</span>
        </p>
      </div>
    </div>
  );
}
