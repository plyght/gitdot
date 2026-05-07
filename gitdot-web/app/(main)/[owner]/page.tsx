import { getOrganization, getUser } from "@/dal";
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

  return <div className="p-2 text-sm font-mono">{owner} not found</div>;
}
