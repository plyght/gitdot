import { z } from "zod";
import { RepositoryBlobPairResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryCommitBlobsRequest = z.object({});
export type GetRepositoryCommitBlobsRequest = z.infer<
  typeof GetRepositoryCommitBlobsRequest
>;

export const GetRepositoryCommitBlobsResponse = z.array(
  RepositoryBlobPairResource,
);
export type GetRepositoryCommitBlobsResponse = z.infer<
  typeof GetRepositoryCommitBlobsResponse
>;

export const GetRepositoryCommitBlobs = {
  path: "/repository/{owner}/{repo}/commits/{sha}/blobs",
  method: "GET",
  request: GetRepositoryCommitBlobsRequest,
  response: GetRepositoryCommitBlobsResponse,
} as const satisfies Endpoint;
export type GetRepositoryCommitBlobs = typeof GetRepositoryCommitBlobs;
