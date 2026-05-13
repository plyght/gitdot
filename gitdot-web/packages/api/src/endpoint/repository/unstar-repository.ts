import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const UnstarRepositoryRequest = z.object({});
export type UnstarRepositoryRequest = z.infer<typeof UnstarRepositoryRequest>;

export const UnstarRepositoryResponse = z.void();
export type UnstarRepositoryResponse = z.infer<typeof UnstarRepositoryResponse>;

export const UnstarRepository = {
  path: "/repository/{owner}/{repo}/unstar",
  method: "POST",
  request: UnstarRepositoryRequest,
  response: UnstarRepositoryResponse,
} as const satisfies Endpoint;
export type UnstarRepository = typeof UnstarRepository;
