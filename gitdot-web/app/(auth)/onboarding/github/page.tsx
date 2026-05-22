import { notFound } from "next/navigation";
import { getCurrentUser } from "@/dal";
import { GithubPageClient } from "./page.client";

export default async function Page() {
  const current = await getCurrentUser();
  if (!current) notFound();
  return <GithubPageClient username={current.user.name} />;
}
