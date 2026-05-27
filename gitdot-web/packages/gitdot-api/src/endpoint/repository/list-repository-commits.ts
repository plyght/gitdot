import { z } from "zod";
import { page, RepositoryCommitResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListRepositoryCommitsRequest = z.object({
  ref_name: z.string().optional(),
  from: z.iso.datetime().optional(),
  to: z.iso.datetime().optional(),
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListRepositoryCommitsRequest = z.infer<
  typeof ListRepositoryCommitsRequest
>;

export const ListRepositoryCommitsResponse = page(RepositoryCommitResource);
export type ListRepositoryCommitsResponse = z.infer<
  typeof ListRepositoryCommitsResponse
>;

export const ListRepositoryCommits = {
  path: "/repository/{owner}/{repo}/commits",
  method: "GET",
  request: ListRepositoryCommitsRequest,
  response: ListRepositoryCommitsResponse,
} as const satisfies Endpoint;
export type ListRepositoryCommits = typeof ListRepositoryCommits;
