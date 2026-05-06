import type { OrganizationResource } from "gitdot-api";
import Link from "@/ui/link";

export function UserOrgs({ orgs }: { orgs: OrganizationResource[] }) {
  return (
    <div className="flex flex-col items-end">
      <p className="font-semibold text-sm mb-0.5">orgs</p>
      {orgs.length ? (
        orgs.map((org) => (
          <Link
            key={org.id}
            href={`/${org.name}`}
            className="text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200"
          >
            {org.name}
          </Link>
        ))
      ) : (
        <span className="text-xs text-muted-foreground">no orgs</span>
      )}
    </div>
  );
}
