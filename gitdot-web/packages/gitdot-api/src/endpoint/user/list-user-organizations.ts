import { z } from "zod";
import { page, UserOrganizationResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListUserOrganizationsRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListUserOrganizationsRequest = z.infer<
  typeof ListUserOrganizationsRequest
>;

export const ListUserOrganizationsResponse = page(UserOrganizationResource);
export type ListUserOrganizationsResponse = z.infer<
  typeof ListUserOrganizationsResponse
>;

export const ListUserOrganizations = {
  path: "/user/{user_name}/organizations",
  method: "GET",
  request: ListUserOrganizationsRequest,
  response: ListUserOrganizationsResponse,
} as const satisfies Endpoint;
export type ListUserOrganizations = typeof ListUserOrganizations;
