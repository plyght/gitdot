import "server-only";

import {
  BuildResource,
  type CreateBuildRequest,
  ListBuildsResponse,
} from "gitdot-api";
import {
  authFetch,
  authPost,
  GITDOT_SERVER_URL,
  handleResponse,
  toQueryString,
} from "./util";

export async function getBuilds(
  owner: string,
  repo: string,
  opts?: { cursor?: string; limit?: number },
): Promise<ListBuildsResponse | null> {
  const qs = toQueryString({ cursor: opts?.cursor, limit: opts?.limit });
  const url = `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/builds${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListBuildsResponse);
}

export async function createBuild(
  owner: string,
  repo: string,
  request: CreateBuildRequest,
): Promise<BuildResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/build`,
    request,
  );

  return await handleResponse(response, BuildResource);
}

export async function getBuild(
  owner: string,
  repo: string,
  number: number,
): Promise<BuildResource | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/build/${number}`,
  );

  return await handleResponse(response, BuildResource);
}
