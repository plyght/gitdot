import { getCurrentUser } from "gitdot-client";
import { notFound } from "next/navigation";
import InstallGithubAppForm from "../../ui/install-github-app-form";
import MigrateRepositoriesForm from "../../ui/migrate-repositories-form";

export default async function Page({
  searchParams,
}: {
  searchParams: Promise<{ installation_id?: string }>;
}) {
  const current = await getCurrentUser();
  if (!current) notFound();

  const { installation_id } = await searchParams;
  const parsed = installation_id ? Number(installation_id) : Number.NaN;
  const installationId = Number.isFinite(parsed) ? parsed : null;

  const username = current.name;
  if (installationId !== null) {
    return (
      <MigrateRepositoriesForm
        username={username}
        installationId={installationId}
      />
    );
  }
  return <InstallGithubAppForm username={username} />;
}
