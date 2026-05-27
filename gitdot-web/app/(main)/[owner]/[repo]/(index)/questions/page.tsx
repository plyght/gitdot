import { listQuestions } from "gitdot-client";
import { PageClient } from "./page.client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const result = await listQuestions(owner, repo);
  const questions = result?.data ?? null;
  return <PageClient owner={owner} repo={repo} questions={questions} />;
}
