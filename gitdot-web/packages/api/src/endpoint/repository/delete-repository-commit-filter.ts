import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const DeleteRepositoryCommitFilterRequest = z.object({});
export type DeleteRepositoryCommitFilterRequest = z.infer<
  typeof DeleteRepositoryCommitFilterRequest
>;

export const DeleteRepositoryCommitFilterResponse = z.void();
export type DeleteRepositoryCommitFilterResponse = z.infer<
  typeof DeleteRepositoryCommitFilterResponse
>;

export const DeleteRepositoryCommitFilter = {
  path: "/repository/{owner}/{repo}/commit-filters/{filter_id}",
  method: "DELETE",
  request: DeleteRepositoryCommitFilterRequest,
  response: DeleteRepositoryCommitFilterResponse,
} as const satisfies Endpoint;
export type DeleteRepositoryCommitFilter = typeof DeleteRepositoryCommitFilter;
