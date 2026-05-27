import { z } from "zod";
import { page, RunnerResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListRunnersRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListRunnersRequest = z.infer<typeof ListRunnersRequest>;

export const ListRunnersResponse = page(RunnerResource);
export type ListRunnersResponse = z.infer<typeof ListRunnersResponse>;

export const ListRunners = {
  path: "/ci/runner/{owner}",
  method: "GET",
  request: ListRunnersRequest,
  response: ListRunnersResponse,
} as const satisfies Endpoint;
export type ListRunners = typeof ListRunners;
