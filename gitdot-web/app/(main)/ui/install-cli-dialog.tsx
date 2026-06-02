"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { Copy } from "lucide-react";
import { type ReactNode, useEffect, useState } from "react";
import { toast } from "@/(main)/context/toaster";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

const INSTALL_COMMAND = "cargo install gitdot-cli";

export function InstallCliLink({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) {
  return (
    <button
      type="button"
      className={className}
      onClick={() => window.dispatchEvent(new CustomEvent("openInstallCli"))}
    >
      {children}
    </button>
  );
}

export function InstallCliDialog() {
  const [open, setOpen] = useState(false);

  useEffect(() => {
    const handle = () => setOpen(true);
    window.addEventListener("openInstallCli", handle);
    return () => window.removeEventListener("openInstallCli", handle);
  }, []);

  const copy = (text: string) => {
    navigator.clipboard.writeText(text);
    toast.success("Copied to clipboard");
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="gap-0! overflow-hidden max-w-xl top-[45%] p-0!"
        animations
        showOverlay
      >
        <VisuallyHidden>
          <DialogTitle>Install gitdot-cli</DialogTitle>
        </VisuallyHidden>

        <div className="px-2 pt-2.5 pb-1">
          <h2 className="font-mono text-sm">gitdot-cli</h2>
          <p className="mt-1 text-xs text-muted-foreground leading-normal">
            Login from your terminal so{" "}
            <span className="font-mono text-foreground">git push</span> and{" "}
            <span className="font-mono text-foreground">git clone</span> works.
            <br />
            The binary is named{" "}
            <span className="font-mono text-foreground">dot</span>.
          </p>
        </div>

        <button
          type="button"
          onClick={() => copy(INSTALL_COMMAND)}
          aria-label="Copy install command"
          className="group flex items-center w-full border-b border-border text-left cursor-pointer"
        >
          <span className="flex-1 min-w-0 px-2 py-2 font-mono text-sm whitespace-nowrap overflow-x-auto">
            {INSTALL_COMMAND}
          </span>
          <span className="flex items-center justify-center px-2 self-stretch text-muted-foreground group-hover:text-foreground transition-colors">
            <Copy className="size-3.5" />
          </span>
        </button>

        <div className="px-2 py-3">
          <h3 className="font-mono text-xs text-muted-foreground mb-1">
            # commands
          </h3>

          <button
            type="button"
            onClick={() => copy("dot login")}
            aria-label="Copy dot login"
            className="block w-fit font-mono text-sm cursor-pointer hover:text-muted-foreground transition-colors"
          >
            dot login
          </button>
          <p className="mt-1 mb-4 text-xs text-muted-foreground leading-normal">
            Authenticate via the OAuth device-code flow. <br />
            Opens a verification URL and waits for you to enter a one-time code
            in your browser. On success your credentials are stored so{" "}
            <span className="font-mono text-foreground">git push</span>{" "}
            authenticates automatically.
          </p>

          <button
            type="button"
            onClick={() => copy("dot status")}
            aria-label="Copy dot status"
            className="block w-fit font-mono text-sm cursor-pointer hover:text-muted-foreground transition-colors"
          >
            dot status
          </button>
          <p className="mt-1 text-xs text-muted-foreground leading-normal">
            Prints the currently logged-in user, or "Not logged in" if no
            session is active.
          </p>
        </div>
      </DialogContent>
    </Dialog>
  );
}
