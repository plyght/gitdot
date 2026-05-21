"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type {
  OrganizationMemberResource,
  RepositoryResource,
  UserResource,
} from "gitdot-api";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { useShortcuts } from "@/(main)/provider/shortcuts";
import { useUserContext } from "@/(main)/provider/user";
import {
  listOrganizationRepositoriesAction,
  listUserRepositoriesAction,
} from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function ReposDialog() {
  const { user, memberships } = useUserContext();
  if (!user || memberships === undefined) return null;

  return <ReposDialogInner user={user} memberships={memberships ?? []} />;
}

function ReposDialogInner({
  user,
  memberships,
}: {
  user: UserResource;
  memberships: OrganizationMemberResource[];
}) {
  const router = useRouter();
  const [open, setOpen] = useState(false);
  const [repositories, setRepositories] = useState<RepositoryResource[] | null>(
    null,
  );

  useShortcuts([
    {
      name: "Repos",
      description: "Open repositories",
      keys: ["r"],
      execute: () => setOpen(true),
    },
  ]);

  useEffect(() => {
    const handle = () => setOpen(true);
    window.addEventListener("openRepos", handle);
    return () => window.removeEventListener("openRepos", handle);
  }, []);

  useEffect(() => {
    Promise.all([
      listUserRepositoriesAction(user.name),
      ...memberships.map((m) => listOrganizationRepositoriesAction(m.org_name)),
    ]).then((results) => setRepositories(results.flat()));
  }, [user, memberships]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-xl min-w-xl border-black top-[35%] p-0 overflow-hidden"
        animations={false}
        showOverlay={false}
      >
        <VisuallyHidden>
          <DialogTitle>Repositories</DialogTitle>
        </VisuallyHidden>
        <div className="flex flex-col max-h-96 overflow-y-auto">
          {repositories === null ? (
            <div className="p-3 text-xs font-mono text-muted-foreground">
              loading...
            </div>
          ) : repositories.length === 0 ? (
            <div className="p-3 text-xs font-mono text-muted-foreground">
              no repos
            </div>
          ) : (
            repositories.map((repo) => (
              <button
                key={`${repo.owner}/${repo.name}`}
                type="button"
                onClick={() => {
                  setOpen(false);
                  router.push(`/${repo.owner}/${repo.name}`);
                }}
                className="flex flex-col gap-0.5 p-2 text-left border-b border-border last:border-b-0 hover:bg-muted/40 transition-colors cursor-pointer"
              >
                <span className="text-sm font-mono">
                  {repo.owner}/{repo.name}
                </span>
                {repo.description && (
                  <span className="text-xs text-muted-foreground">
                    {repo.description}
                  </span>
                )}
              </button>
            ))
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
