"use client";

import type { OrganizationMemberResource } from "gitdot-api";
import { ChevronDown } from "lucide-react";
import { useMemo, useState } from "react";
import { useTimezone } from "@/(main)/context/timezone";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { formatDate } from "@/util/date";
import { UserImage } from "../user/user-image";

type MemberSort = "newest" | "oldest";

const MEMBER_SORT_LABELS: Record<MemberSort, string> = {
  newest: "Newest",
  oldest: "Oldest",
};

export function OrgMembers({
  members,
}: {
  members: OrganizationMemberResource[] | null;
}) {
  const tz = useTimezone();
  const [sortBy, setSortBy] = useState<MemberSort>("oldest");

  const sortedMembers = useMemo(() => {
    if (!members) return [];
    return [...members].sort((a, b) => {
      const aTime = new Date(a.created_at).getTime();
      const bTime = new Date(b.created_at).getTime();
      return sortBy === "newest" ? bTime - aTime : aTime - bTime;
    });
  }, [members, sortBy]);

  if (!members?.length) return null;

  return (
    <div className="px-3">
      <div className="flex items-baseline justify-between mb-2">
        <span className="text-xs text-muted-foreground font-mono">
          <span className="text-foreground/40 select-none"># </span>
          Members
        </span>
        <DropdownMenu>
          <DropdownMenuTrigger className="flex items-center gap-0.5 text-xs text-muted-foreground/60 font-mono cursor-pointer transition-colors hover:text-foreground">
            {MEMBER_SORT_LABELS[sortBy]}
            <ChevronDown className="size-3" />
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" className="min-w-20">
            {(Object.keys(MEMBER_SORT_LABELS) as MemberSort[]).map((key) => (
              <DropdownMenuItem
                key={key}
                className="text-xs font-mono"
                onClick={() => setSortBy(key)}
              >
                {MEMBER_SORT_LABELS[key]}
              </DropdownMenuItem>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      <div className="flex flex-col gap-4">
        {sortedMembers.map((member) => (
          <Link
            key={member.id}
            href={`/${member.user_name}`}
            className="group flex items-start gap-x-3 w-full"
          >
            <UserImage
              userId={member.user_id}
              updatedAt={member.image_updated_at}
              px={32}
            />
            <div className="flex flex-col min-w-0 flex-1">
              <div className="flex items-baseline justify-between gap-3">
                <span className="text-sm font-medium dark:font-normal underline decoration-transparent group-hover:decoration-current group-focus-within:decoration-current transition-colors duration-200">
                  {member.user_name}
                </span>
                <span className="text-xs font-mono text-muted-foreground whitespace-nowrap shrink-0">
                  joined {formatDate(new Date(member.created_at), tz)}
                </span>
              </div>
              <p
                className={
                  member.role_description
                    ? "text-xs text-foreground"
                    : "text-xs text-muted-foreground italic"
                }
              >
                {member.role_description || "no description"}
              </p>
            </div>
          </Link>
        ))}
      </div>
    </div>
  );
}
