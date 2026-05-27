import { z } from "zod";
import { page, UserCommitResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListUserCommitsRequest = z.object({
  from: z.iso.datetime().optional(),
  to: z.iso.datetime().optional(),
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListUserCommitsRequest = z.infer<typeof ListUserCommitsRequest>;

export const ListUserCommitsResponse = page(UserCommitResource);
export type ListUserCommitsResponse = z.infer<typeof ListUserCommitsResponse>;

export const ListUserCommits = {
  path: "/user/{user_name}/commits",
  method: "GET",
  request: ListUserCommitsRequest,
  response: ListUserCommitsResponse,
} as const satisfies Endpoint;
export type ListUserCommits = typeof ListUserCommits;
