import {
  getUser,
  listUserCommits,
  listUserOrganizations,
  listUserRepositories,
} from "@/dal";
import { UserActions } from "./user-actions";
import { UserCommits } from "./user-commits";
import { UserLinks } from "./user-links";
import { UserOrgs } from "./user-orgs";
import { UserProfile } from "./user-profile";
import { UserReadme } from "./user-readme";
import { UserRepos } from "./user-repos";

export default async function UserPage({ username }: { username: string }) {
  const [user, commits, repos, orgs] = await Promise.all([
    getUser(username),
    listUserCommits(username),
    listUserRepositories(username),
    listUserOrganizations(username),
  ]);

  if (!user) {
    return <div className="p-2 text-sm">{username} not found</div>;
  }

  return (
    <div className="grid grid-cols-[8rem_1fr_1fr] items-start py-4 h-full overflow-y-auto scrollbar-thin">
      <div className="flex flex-col items-end px-4 gap-6">
        <UserProfile user={user} />
        <UserLinks user={user} />
        <UserActions />
      </div>

      <div className="border-x pl-4 pr-6 flex flex-col gap-8">
        <UserReadme readme={user.readme} />
        <UserOrgs orgs={orgs} />
        <UserRepos repos={repos} />
      </div>

      <div className="px-4">
        <UserCommits commits={commits ?? []} />
      </div>
    </div>
  );
}
