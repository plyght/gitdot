import { z } from "zod";

import { RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryRequest = z.object({});
export type GetRepositoryRequest = z.infer<typeof GetRepositoryRequest>;

export const GetRepositoryResponse = RepositoryResource;
export type GetRepositoryResponse = z.infer<typeof GetRepositoryResponse>;

export const GetRepository = {
  path: "/repository/{owner}/{repo}",
  method: "GET",
  request: GetRepositoryRequest,
  response: GetRepositoryResponse,
} as const satisfies Endpoint;
export type GetRepository = typeof GetRepository;
