import { z } from "zod";
import { BuildResource, page } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListBuildsRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListBuildsRequest = z.infer<typeof ListBuildsRequest>;

export const ListBuildsResponse = page(BuildResource);
export type ListBuildsResponse = z.infer<typeof ListBuildsResponse>;

export const ListBuilds = {
  path: "/repository/{owner}/{repo}/builds",
  method: "GET",
  request: ListBuildsRequest,
  response: ListBuildsResponse,
} as const satisfies Endpoint;
export type ListBuilds = typeof ListBuilds;
