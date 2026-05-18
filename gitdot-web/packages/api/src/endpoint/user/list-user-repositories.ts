import { z } from "zod";
import { page, RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListUserRepositoriesRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListUserRepositoriesRequest = z.infer<
  typeof ListUserRepositoriesRequest
>;

export const ListUserRepositoriesResponse = page(RepositoryResource);
export type ListUserRepositoriesResponse = z.infer<
  typeof ListUserRepositoriesResponse
>;

export const ListUserRepositories = {
  path: "/user/{user_name}/repositories",
  method: "GET",
  request: ListUserRepositoriesRequest,
  response: ListUserRepositoriesResponse,
} as const satisfies Endpoint;
export type ListUserRepositories = typeof ListUserRepositories;
