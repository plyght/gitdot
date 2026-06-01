"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { ClientProvider } from "gitdot-dal/client";
import { useRouter } from "next/navigation";
import { useEffect, useState, useTransition } from "react";
import { toast } from "@/(main)/context/toaster";
import { useUserContext } from "@/(main)/context/user";
import { deleteAccountAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function UserDeleteAccountDialog({
  open,
  setOpen,
  setUserSettingsOpen,
  username,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  setUserSettingsOpen: (open: boolean) => void;
  username: string;
}) {
  const router = useRouter();
  const { refreshUser } = useUserContext();
  const [confirmation, setConfirmation] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();

  useEffect(() => {
    if (open) {
      setConfirmation("");
      setError(null);
    }
  }, [open]);

  const isValid = confirmation === username && username.length > 0;

  function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    if (!isValid || isPending) return;

    setError(null);
    startTransition(async () => {
      const result = await deleteAccountAction();
      if ("error" in result) {
        setError(result.error);
        return;
      }
      toast.success("Your account has been deleted");
      setOpen(false);
      setUserSettingsOpen(false);
      await ClientProvider.instance.invalidate();
      await refreshUser();
      router.push("/");
    });
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-md min-w-md border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>Delete account</DialogTitle>
        </VisuallyHidden>
        <form onSubmit={handleSubmit}>
          <div className="p-2 border-b border-border flex flex-col gap-2">
            <p className="text-sm">Delete your account?</p>
            <p className="text-xs text-muted-foreground">
              This permanently removes your account and personal data and signs
              you out everywhere. This action cannot be undone.
            </p>
            <p className="text-xs text-muted-foreground">
              Type <span className="font-medium">{username}</span> below to
              confirm.
            </p>
            {error && <p className="text-xs text-red-500 px-2 py-1">{error}</p>}
          </div>
          <div className="flex h-7">
            <input
              type="text"
              name="confirmation"
              placeholder={username}
              value={confirmation}
              onChange={(event) => setConfirmation(event.target.value)}
              className="flex-1 min-w-0 h-full px-2 text-xs leading-7 border-0 outline-none pb-1"
              disabled={isPending}
              autoFocus
            />
            <button
              type="submit"
              disabled={!isValid || isPending}
              className="flex items-center px-3 h-full text-xs bg-destructive text-white border-l border-destructive enabled:hover:opacity-90 disabled:opacity-60 transition-opacity disabled:cursor-not-allowed cursor-pointer"
            >
              {isPending ? "Deleting..." : "Delete"}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
