import { z } from "zod";
import { OrganizationResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetOrganizationRequest = z.object({});
export type GetOrganizationRequest = z.infer<typeof GetOrganizationRequest>;

export const GetOrganizationResponse = OrganizationResource;
export type GetOrganizationResponse = z.infer<typeof GetOrganizationResponse>;

export const GetOrganization = {
  path: "/organization/{org_name}",
  method: "GET",
  request: GetOrganizationRequest,
  response: GetOrganizationResponse,
} as const satisfies Endpoint;
export type GetOrganization = typeof GetOrganization;
