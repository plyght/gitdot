import "server-only";

import {
  ListOrganizationMembersResponse,
  ListOrganizationRepositoriesResponse,
  OrganizationMemberResource,
  OrganizationResource,
} from "gitdot-api";
import { toQueryString } from "@/util";
import {
  authFetch,
  authPatch,
  authPost,
  GITDOT_SERVER_URL,
  handleResponse,
} from "./util";

export async function getOrganization(
  name: string,
): Promise<OrganizationResource | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/organization/${name}`);
  return await handleResponse(response, OrganizationResource);
}

export async function listOrganizationMembers(
  name: string,
  opts?: { role?: string; cursor?: string; limit?: number },
): Promise<ListOrganizationMembersResponse | null> {
  const qs = toQueryString({
    role: opts?.role,
    cursor: opts?.cursor,
    limit: opts?.limit,
  });
  const url = `${GITDOT_SERVER_URL}/organization/${name}/members${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListOrganizationMembersResponse);
}

export async function addOrganizationMember(
  orgName: string,
  request: {
    user_name: string;
    role: string;
    role_description?: string | null;
  },
): Promise<OrganizationMemberResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/organization/${orgName}/member`,
    request,
  );
  return await handleResponse(response, OrganizationMemberResource);
}

export async function updateOrganizationMember(
  orgName: string,
  memberId: string,
  request: { role_description: string | null },
): Promise<OrganizationMemberResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/organization/${orgName}/member/${memberId}`,
    request,
  );
  return await handleResponse(response, OrganizationMemberResource);
}

export async function listOrganizationRepositories(
  name: string,
  opts?: { cursor?: string; limit?: number },
): Promise<ListOrganizationRepositoriesResponse | null> {
  const qs = toQueryString({ cursor: opts?.cursor, limit: opts?.limit });
  const url = `${GITDOT_SERVER_URL}/organization/${name}/repositories${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListOrganizationRepositoriesResponse);
}

export async function createOrganization(
  name: string,
  readme?: string,
): Promise<OrganizationResource | null> {
  const response = await authPost(`${GITDOT_SERVER_URL}/organization/${name}`, {
    readme,
  });
  return await handleResponse(response, OrganizationResource);
}

export async function updateOrganization(
  name: string,
  request: {
    location?: string | null;
    readme?: string | null;
    links?: string[];
    display_name?: string | null;
  },
): Promise<OrganizationResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/organization/${name}`,
    request,
  );
  return await handleResponse(response, OrganizationResource);
}

export async function uploadOrganizationImage(
  orgName: string,
  file: File,
): Promise<boolean> {
  const bytes = await file.arrayBuffer();
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/organization/${orgName}/image`,
    {
      method: "POST",
      headers: { "Content-Type": file.type },
      body: bytes,
    },
  );
  return response.ok;
}
