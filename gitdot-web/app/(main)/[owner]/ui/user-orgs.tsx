import type { OrganizationResource } from "gitdot-api";
import Link from "@/ui/link";

export function UserOrgs({ orgs }: { orgs: OrganizationResource[] }) {
  if (!orgs.length) return null;

  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-2">
        <span className="text-foreground/40 select-none"># </span>
        Organizations
      </p>
      <div className="flex flex-col gap-1">
        {orgs.map((org) => (
          <Link
            key={org.id}
            href={`/${org.name}`}
            className="text-sm font-medium underline decoration-transparent hover:decoration-current transition-colors duration-200 self-start"
          >
            {org.name}
          </Link>
        ))}
      </div>
    </div>
  );
}
