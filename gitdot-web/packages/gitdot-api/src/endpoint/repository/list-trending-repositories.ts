import { z } from "zod";
import { RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListTrendingRepositoriesRequest = z.object({});
export type ListTrendingRepositoriesRequest = z.infer<
  typeof ListTrendingRepositoriesRequest
>;

export const ListTrendingRepositoriesResponse = z.array(RepositoryResource);
export type ListTrendingRepositoriesResponse = z.infer<
  typeof ListTrendingRepositoriesResponse
>;

export const ListTrendingRepositories = {
  path: "/repository/trending",
  method: "GET",
  request: ListTrendingRepositoriesRequest,
  response: ListTrendingRepositoriesResponse,
} as const satisfies Endpoint;
export type ListTrendingRepositories = typeof ListTrendingRepositories;
