import { z } from "zod";
import { page, RepositoryCommitFilterResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListRepositoryCommitFiltersRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListRepositoryCommitFiltersRequest = z.infer<
  typeof ListRepositoryCommitFiltersRequest
>;

export const ListRepositoryCommitFiltersResponse = page(
  RepositoryCommitFilterResource,
);
export type ListRepositoryCommitFiltersResponse = z.infer<
  typeof ListRepositoryCommitFiltersResponse
>;

export const ListRepositoryCommitFilters = {
  path: "/repository/{owner}/{repo}/commit-filters",
  method: "GET",
  request: ListRepositoryCommitFiltersRequest,
  response: ListRepositoryCommitFiltersResponse,
} as const satisfies Endpoint;
export type ListRepositoryCommitFilters = typeof ListRepositoryCommitFilters;
