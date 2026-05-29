import { getRepositoryCommitBlobs } from "gitdot-client";
import type { NextRequest } from "next/server";

export async function GET(request: NextRequest) {
  const { searchParams } = request.nextUrl;
  const owner = searchParams.get("owner") ?? "";
  const repo = searchParams.get("repo") ?? "";
  const sha = searchParams.get("sha") ?? "";
  const pairs = await getRepositoryCommitBlobs(owner, repo, sha);
  return Response.json(pairs ?? []);
}
