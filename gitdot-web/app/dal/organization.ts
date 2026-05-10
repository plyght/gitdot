import "server-only";

import { OrganizationResource } from "gitdot-api";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function getOrganization(
  name: string,
): Promise<OrganizationResource | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/organization/${name}`);
  return await handleResponse(response, OrganizationResource);
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
