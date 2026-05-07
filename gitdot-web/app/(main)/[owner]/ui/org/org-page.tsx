import type { OrganizationResource } from "gitdot-api";

export default function OrgPage({ org }: { org: OrganizationResource }) {
  return (
    <div className="p-4 text-sm">
      <h1 className="text-lg font-medium">{org.name}</h1>
      <p className="text-muted-foreground">Organization page (placeholder)</p>
    </div>
  );
}
