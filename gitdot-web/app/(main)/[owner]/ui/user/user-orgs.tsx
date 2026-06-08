"use client";

import type { UserOrganizationResource } from "gitdot-api";
import { useTimezone } from "@/(main)/context/timezone";
import Link from "@/ui/link";
import { formatDate } from "@/util/date";
import { OrgImage } from "../org/org-image";

export function UserOrgs({
  memberships,
}: {
  memberships: UserOrganizationResource[] | null;
}) {
  const tz = useTimezone();
  if (!memberships?.length) return null;

  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-2">
        <span className="text-foreground/40 select-none"># </span>
        Organizations
      </p>
      <div className="flex flex-col gap-4">
        {memberships.map((m) => (
          <Link
            key={m.id}
            href={`/${m.name}`}
            className="group flex items-start gap-x-3 w-full"
          >
            <OrgImage orgId={m.id} updatedAt={m.image_updated_at} px={32} />
            <div className="flex flex-col min-w-0 flex-1">
              <div className="flex items-baseline justify-between gap-3">
                <span className="text-sm font-medium dark:font-normal underline decoration-transparent group-hover:decoration-current group-focus-within:decoration-current transition-colors duration-200">
                  {m.name}
                </span>
                <span className="text-xs font-mono text-muted-foreground whitespace-nowrap shrink-0">
                  member since {formatDate(new Date(m.joined_at), tz)}
                </span>
              </div>
              <p
                className={
                  m.role_description
                    ? "text-xs text-foreground"
                    : "text-xs text-muted-foreground italic"
                }
              >
                {m.role_description || "no description"}
              </p>
            </div>
          </Link>
        ))}
      </div>
    </div>
  );
}
