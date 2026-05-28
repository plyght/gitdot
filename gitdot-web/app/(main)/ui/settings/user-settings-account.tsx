"use client";

import { useState, useTransition } from "react";
import { toast } from "@/(main)/context/toaster";
import { useUserContext } from "@/(main)/context/user";
import { resendUserEmailAction } from "@/actions";
import { cn } from "@/util";
import { UserAddEmailDialog } from "./user-add-email-dialog";
import { UserChangeNameDialog } from "./user-change-name-dialog";

export function UserSettingsAccount({
  setUserSettingsOpen,
}: {
  setUserSettingsOpen: (open: boolean) => void;
}) {
  const [changeOpen, setChangeOpen] = useState(false);
  const [addEmailOpen, setAddEmailOpen] = useState(false);
  const [prefillEmail, setPrefillEmail] = useState<string | undefined>(
    undefined,
  );

  function openAddEmail(prefill?: string) {
    setPrefillEmail(prefill);
    setAddEmailOpen(true);
  }

  return (
    <>
      <div className="divide-y divide-border">
        <AccountAction
          title="Change username"
          description="Pick a new handle. Your old username becomes available for anyone else to claim, links pointing to your old profile will break, and every existing git remote must be updated to the new URL."
          actionLabel="Change"
          onAction={() => setChangeOpen(true)}
        />
        <EmailsSection onAddEmail={openAddEmail} />
        <AccountAction
          title="Delete account"
          description="Permanently remove your account, repositories, and personal data. This cannot be undone."
          actionLabel="Delete"
          destructive
          onAction={() => {}}
        />
      </div>
      <UserChangeNameDialog
        open={changeOpen}
        setOpen={setChangeOpen}
        setUserSettingsOpen={setUserSettingsOpen}
      />
      <UserAddEmailDialog
        open={addEmailOpen}
        setOpen={setAddEmailOpen}
        initialEmail={prefillEmail}
      />
    </>
  );
}

function EmailsSection({
  onAddEmail,
}: {
  onAddEmail: (prefill?: string) => void;
}) {
  const { emails } = useUserContext();

  const verified = (emails ?? [])
    .filter((e) => e.is_verified)
    .sort((a, b) => {
      if (a.is_primary !== b.is_primary) return a.is_primary ? -1 : 1;
      return 0;
    });
  const pending = (emails ?? []).filter((e) => !e.is_verified);

  return (
    <div className="p-3">
      <p className="text-sm font-medium dark:font-normal">Manage emails</p>
      <p className="text-sm text-muted-foreground">
        Manage your emails. gitdot attributes commits to the emails listed here
        — match one to your git config.
      </p>
      {(verified.length > 0 || pending.length > 0) && (
        <div className="mt-3 divide-y divide-border">
          {verified.map((e) => (
            <div
              key={e.email}
              className="grid grid-cols-[1fr_auto] items-center gap-x-3 py-1 h-7"
            >
              <span className="text-sm truncate">{e.email}</span>
              <span className="text-xs text-muted-foreground font-mono">
                {e.is_primary ? "primary" : "verified"}
              </span>
            </div>
          ))}
          {pending.map((e) => (
            <PendingRow key={e.email} email={e.email} onAddEmail={onAddEmail} />
          ))}
        </div>
      )}
      <div className="flex justify-start mt-3">
        <button
          type="button"
          onClick={() => onAddEmail()}
          className="text-sm underline underline-offset-2 cursor-pointer transition-colors text-muted-foreground hover:text-foreground"
        >
          Add email
        </button>
      </div>
    </div>
  );
}

function PendingRow({
  email,
  onAddEmail,
}: {
  email: string;
  onAddEmail: (prefill?: string) => void;
}) {
  const [isPending, startTransition] = useTransition();

  function handleResend() {
    if (isPending) return;
    startTransition(async () => {
      const result = await resendUserEmailAction(email);
      if ("error" in result) {
        toast.error(result.error);
        return;
      }
      onAddEmail(email);
    });
  }

  return (
    <div className="grid grid-cols-[1fr_auto] items-center gap-x-3 py-1 h-7">
      <span className="text-sm truncate text-muted-foreground">{email}</span>
      <button
        type="button"
        onClick={handleResend}
        disabled={isPending}
        className="text-xs underline underline-offset-2 text-foreground transition-colors cursor-pointer disabled:cursor-not-allowed"
      >
        resend code
      </button>
    </div>
  );
}

function AccountAction({
  title,
  description,
  actionLabel,
  destructive = false,
  onAction,
}: {
  title: string;
  description: string;
  actionLabel: string;
  destructive?: boolean;
  onAction: () => void;
}) {
  return (
    <div className="p-3">
      <p className="text-sm font-medium dark:font-normal">{title}</p>
      <p className="text-sm text-muted-foreground">{description}</p>
      <div className="flex justify-start mt-3">
        <button
          type="button"
          onClick={onAction}
          className={cn(
            "text-sm underline underline-offset-2 cursor-pointer transition-colors",
            destructive
              ? "text-destructive hover:text-destructive/80"
              : "text-muted-foreground hover:text-foreground",
          )}
        >
          {actionLabel}
        </button>
      </div>
    </div>
  );
}
