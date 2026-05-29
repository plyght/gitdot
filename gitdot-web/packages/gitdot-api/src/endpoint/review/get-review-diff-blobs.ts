import { z } from "zod";
import { RepositoryBlobPairResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetReviewDiffBlobsResponse = z.array(RepositoryBlobPairResource);
export type GetReviewDiffBlobsResponse = z.infer<
  typeof GetReviewDiffBlobsResponse
>;

export const GetReviewDiffBlobs = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}/blobs",
  method: "GET",
  request: z.object({}),
  response: GetReviewDiffBlobsResponse,
} as const satisfies Endpoint;
export type GetReviewDiffBlobs = typeof GetReviewDiffBlobs;
