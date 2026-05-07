import UserPage from "./ui/user/user-page";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  return <UserPage username={owner} />;
}
