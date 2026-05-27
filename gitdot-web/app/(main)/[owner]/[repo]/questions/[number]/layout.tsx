import { listQuestions } from "gitdot-client";
import { LayoutClient } from "./layout.client";

export default async function Layout({
  params,
  children,
}: {
  params: Promise<{ owner: string; repo: string; number: string }>;
  children: React.ReactNode;
}) {
  const { owner, repo } = await params;
  const result = await listQuestions(owner, repo);
  const questions = result?.data ?? null;

  return (
    <LayoutClient owner={owner} repo={repo} questions={questions}>
      {children}
    </LayoutClient>
  );
}
