import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const StarRepositoryRequest = z.object({});
export type StarRepositoryRequest = z.infer<typeof StarRepositoryRequest>;

export const StarRepositoryResponse = z.void();
export type StarRepositoryResponse = z.infer<typeof StarRepositoryResponse>;

export const StarRepository = {
  path: "/repository/{owner}/{repo}/star",
  method: "POST",
  request: StarRepositoryRequest,
  response: StarRepositoryResponse,
} as const satisfies Endpoint;
export type StarRepository = typeof StarRepository;
