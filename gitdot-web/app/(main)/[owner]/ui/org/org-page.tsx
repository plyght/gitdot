import type { OrganizationResource } from "gitdot-api";
import { listOrganizationMembers } from "@/dal";
import { OrgActions } from "./org-actions";
import { OrgLinks } from "./org-links";
import { OrgMembers } from "./org-members";
import { OrgProfile } from "./org-profile";
import { OrgReadme } from "./org-readme";

export default async function OrgPage({ org }: { org: OrganizationResource }) {
  const members = await listOrganizationMembers(org.name);

  return (
    <div className="grid grid-cols-[8rem_minmax(0,3fr)_minmax(0,2fr)] h-full">
      <div className="overflow-y-auto scrollbar-none">
        <div className="flex flex-col items-end px-4 my-2.5 pt-0.5 gap-6 border-r">
          <OrgProfile org={org} />
          <OrgLinks org={org} />
          <OrgActions />
        </div>
      </div>

      <div className="pl-4 pr-3 py-2 flex flex-col gap-8 overflow-y-auto scrollbar-none">
        <OrgReadme readme={org.readme} />
        <div className="text-sm text-muted-foreground">
          <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
            eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
            ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut
            aliquip ex ea commodo consequat.
          </p>
          <p className="mt-4">
            Duis aute irure dolor in reprehenderit in voluptate velit esse
            cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
            cupidatat non proident, sunt in culpa qui officia deserunt mollit
            anim id est laborum.
          </p>
          <p className="mt-4">
            Sed ut perspiciatis unde omnis iste natus error sit voluptatem
            accusantium doloremque laudantium, totam rem aperiam, eaque ipsa
            quae ab illo inventore veritatis et quasi architecto beatae vitae
            dicta sunt explicabo.
          </p>
        </div>
      </div>

      <div className="pt-2 pl-4 pr-3 py-2 border-l flex flex-col min-h-0 overflow-y-auto scrollbar-none">
        <OrgMembers members={members} />
      </div>
    </div>
  );
}
