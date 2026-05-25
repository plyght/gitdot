import { getOrganization, getUser } from "@/dal";
import Link from "@/ui/link";

import OrgPage from "./ui/org/org-page";
import UserPage from "./ui/user/user-page";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  const [user, org] = await Promise.all([
    getUser(owner),
    getOrganization(owner),
  ]);

  if (user && org) {
    throw new Error(
      `slug "${owner}" resolved to both a user and an organization`,
    );
  }

  if (user) return <UserPage user={user} />;
  if (org) return <OrgPage org={org} />;

  return (
    <div className="flex flex-col items-center justify-center h-full w-full gap-1 p-4">
      <p className="text-sm font-mono text-foreground">{owner} not found</p>
      <Link
        href={"/"}
        className="text-xs text-muted-foreground underline lowercase"
      >
        return home
      </Link>
    </div>
  );
}
