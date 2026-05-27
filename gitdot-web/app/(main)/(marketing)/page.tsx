import {
  listLatestRepositories,
  listTrendingRepositories,
} from "gitdot-client";
import { PageClient } from "./page.client";

export default async function Page() {
  const [trending, latest] = await Promise.all([
    listTrendingRepositories(),
    listLatestRepositories(),
  ]);

  return <PageClient trending={trending ?? []} latest={latest ?? []} />;
}
