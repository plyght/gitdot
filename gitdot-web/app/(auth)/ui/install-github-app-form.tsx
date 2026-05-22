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
        <p className="pb-2">Connect GitHub.</p>
        <p className="text-muted-foreground leading-relaxed">
          Import your profile, commit history, and repositories. All code and
          commit history is preserved; private repos stay private.
        </p>
        <p className="pt-2 text-muted-foreground leading-relaxed">
          There are two ways to migrate:
        </p>
        <ul className="text-muted-foreground leading-relaxed list-disc pl-4 flex flex-col gap-0">
          <li>
            <span className="text-foreground">Read-only:</span> a one-way sync.
            New commits on GitHub are replicated to gitdot.
          </li>
          <li>
            <span className="text-foreground">Read-write:</span> a one-time
            migration to a fully functioning gitdot repository.
          </li>
        </ul>
        <p className="pt-2 pb-4 text-muted-foreground leading-relaxed">
          Read-only repositories can be promoted to read-write at any time.
        </p>
        <p className="text-right">
          <button
            type="button"
            onClick={handleInstall}
            disabled={isInstalling}
            className="cursor-pointer underline text-foreground decoration-current transition-colors disabled:cursor-not-allowed"
          >
            Connect
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
