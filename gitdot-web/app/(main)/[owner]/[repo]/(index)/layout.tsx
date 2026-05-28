import { LayoutClient } from "./layout.client";

export default async function Layout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;

  return (
    <LayoutClient owner={owner} repo={repo}>
      {children}
    </LayoutClient>
  );
}
