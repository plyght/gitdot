import type { OrganizationResource } from "gitdot-api";
import { OrgImage } from "./org-image";

export function OrgProfile({ org }: { org: OrganizationResource }) {
  return (
    <div className="flex flex-col items-start">
      <div className="mb-0.5">
        <OrgImage orgId={org.id} />
      </div>
      <p className="font-semibold text-sm mb-0.5">{org.name}</p>
    </div>
  );
}
