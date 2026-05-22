import { z } from "zod";
import { RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListLatestRepositoriesRequest = z.object({});
export type ListLatestRepositoriesRequest = z.infer<
  typeof ListLatestRepositoriesRequest
>;

export const ListLatestRepositoriesResponse = z.array(RepositoryResource);
export type ListLatestRepositoriesResponse = z.infer<
  typeof ListLatestRepositoriesResponse
>;

export const ListLatestRepositories = {
  path: "/repository/latest",
  method: "GET",
  request: ListLatestRepositoriesRequest,
  response: ListLatestRepositoriesResponse,
} as const satisfies Endpoint;
export type ListLatestRepositories = typeof ListLatestRepositories;
