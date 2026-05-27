import { getBuilds, listRepositoryCommits } from "gitdot-client";
import { PageClient } from "./page.client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const [buildsResult, commitsResult] = await Promise.all([
    getBuilds(owner, repo),
    listRepositoryCommits(owner, repo),
  ]);
  const builds = buildsResult?.data ?? null;
  const commits = commitsResult?.data ?? null;

  return (
    <PageClient owner={owner} repo={repo} builds={builds} commits={commits} />
  );
}
