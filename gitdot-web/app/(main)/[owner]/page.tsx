import {
  getUser,
  listUserCommits,
  listUserOrganizations,
  listUserRepositories,
} from "@/dal";
import { UserCommits } from "./ui/user-commits";
import { UserLinks } from "./ui/user-links";
import { UserOrgs } from "./ui/user-orgs";
import { UserProfile } from "./ui/user-profile";
import { UserReadme } from "./ui/user-readme";
import { UserRepos } from "./ui/user-repos";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string }>;
}) {
  const { owner } = await params;
  const [user, commits, repos, orgs] = await Promise.all([
    getUser(owner),
    listUserCommits(owner),
    listUserRepositories(owner),
    listUserOrganizations(owner),
  ]);

  if (!user) {
    return <div className="p-2 text-sm">{owner} not found</div>;
  }

  return (
    <div className="grid grid-cols-[auto_1fr_1fr] items-start py-4 h-full overflow-y-auto scrollbar-thin">
      <div className="flex flex-col items-end px-4 gap-6">
        <UserProfile user={user} />
        <UserOrgs orgs={orgs ?? []} />
        <UserLinks user={user} />
      </div>

      <div className="border-x pl-4 pr-6 flex flex-col gap-8">
        <UserReadme readme={user.readme} />
        <UserRepos owner={owner} repos={repos} />
      </div>

      <div className="px-4">
        <UserCommits commits={commits ?? []} />
      </div>
    </div>
  );
}
