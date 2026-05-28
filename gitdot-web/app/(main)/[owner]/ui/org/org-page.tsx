import type { OrganizationResource } from "gitdot-api";
import { getCurrentUser, listOrganizationRepositories } from "gitdot-client";
import { OrgActions } from "./org-actions";
import { OrgLinks } from "./org-links";
import { OrgMembers } from "./org-members";
import { OrgProfile } from "./org-profile";
import { OrgReadme } from "./org-readme";
import { OrgRepositories } from "./org-repositories";

export default async function OrgPage({ org }: { org: OrganizationResource }) {
  const [reposResponse, current] = await Promise.all([
    listOrganizationRepositories(org.name),
    getCurrentUser(false),
  ]);
  const members = org.members ?? null;
  const repos = reposResponse?.data ?? null;
  const userMembership =
    current?.memberships.find((m) => m.name === org.name) ?? null;
  const role: "guest" | "member" | "admin" = userMembership
    ? userMembership.role === "admin"
      ? "admin"
      : "member"
    : "guest";
  const isMember = role !== "guest";
  const ownMember =
    current && members
      ? (members.find((m) => m.user_id === current.id) ?? null)
      : null;

  return (
    <div className="grid grid-cols-[15rem_minmax(0,3fr)_minmax(0,2fr)] h-full">
      <div className="overflow-y-auto scrollbar-none">
        <div className="flex flex-col items-end pl-2 pr-4 my-2.5 pt-0.5 gap-6">
          <OrgProfile org={org} />
          <OrgLinks org={org} />
          <OrgActions
            org={org}
            members={members}
            role={role}
            membership={ownMember}
          />
        </div>
      </div>

      <div className="px-3 py-2 border-l flex flex-col gap-8 overflow-y-auto scrollbar-none">
        <OrgReadme readme={org.readme} />
        <OrgRepositories repos={repos} isMember={isMember} />
      </div>

      <div className="pt-2 border-l flex flex-col min-h-0">
        <OrgMembers members={members} />
      </div>
    </div>
  );
}
