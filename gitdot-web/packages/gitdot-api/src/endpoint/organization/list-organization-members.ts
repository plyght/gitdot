import { z } from "zod";
import { OrganizationMemberResource, page } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListOrganizationMembersRequest = z.object({
  role: z.string().optional(),
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListOrganizationMembersRequest = z.infer<
  typeof ListOrganizationMembersRequest
>;

export const ListOrganizationMembersResponse = page(OrganizationMemberResource);
export type ListOrganizationMembersResponse = z.infer<
  typeof ListOrganizationMembersResponse
>;

export const ListOrganizationMembers = {
  path: "/organization/{org_name}/members",
  method: "GET",
  request: ListOrganizationMembersRequest,
  response: ListOrganizationMembersResponse,
} as const satisfies Endpoint;
export type ListOrganizationMembers = typeof ListOrganizationMembers;
