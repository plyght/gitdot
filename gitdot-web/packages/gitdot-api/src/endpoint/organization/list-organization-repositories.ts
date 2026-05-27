import { z } from "zod";
import { page, RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListOrganizationRepositoriesRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListOrganizationRepositoriesRequest = z.infer<
  typeof ListOrganizationRepositoriesRequest
>;

export const ListOrganizationRepositoriesResponse = page(RepositoryResource);
export type ListOrganizationRepositoriesResponse = z.infer<
  typeof ListOrganizationRepositoriesResponse
>;

export const ListOrganizationRepositories = {
  path: "/organization/{org_name}/repositories",
  method: "GET",
  request: ListOrganizationRepositoriesRequest,
  response: ListOrganizationRepositoriesResponse,
} as const satisfies Endpoint;
export type ListOrganizationRepositories = typeof ListOrganizationRepositories;
