import { getCurrentUser } from "@/dal";
import { LayoutClient } from "./layout.client";

export default async function Layout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const current = await getCurrentUser(false);
  const isAdmin =
    current?.name === owner ||
    (current?.memberships ?? []).some(
      (m) => m.org_name === owner && m.role === "admin",
    );

  return (
    <LayoutClient owner={owner} repo={repo} showSettings={isAdmin}>
      {children}
    </LayoutClient>
  );
}
