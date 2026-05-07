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
    <div className="grid grid-cols-[8rem_minmax(0,1fr)_minmax(0,1fr)] h-full">
      <div className="overflow-y-auto scrollbar-none">
        <div className="flex flex-col items-end px-4 my-2.5 pt-0.5 gap-6 border-r">
          <UserProfile user={user} />
          <UserLinks user={user} />
          <UserActions />
        </div>
      </div>

      <div className="pl-4 pr-6 py-2 flex flex-col gap-8 overflow-y-auto scrollbar-none">
        <UserReadme readme={user.readme} />
        <UserOrgs orgs={orgs} />
        <UserRepos repos={repos} />
      </div>

      <div className="px-4 pt-2 border-l flex flex-col min-h-0">
        <UserCommits commits={commits ?? []} />
      </div>
    </div>
  );
}
