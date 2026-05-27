import { z } from "zod";
import { MigrationResource, page } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListMigrationsRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListMigrationsRequest = z.infer<typeof ListMigrationsRequest>;

export const ListMigrationsResponse = page(MigrationResource);
export type ListMigrationsResponse = z.infer<typeof ListMigrationsResponse>;

export const ListMigrations = {
  path: "/migrations",
  method: "GET",
  request: ListMigrationsRequest,
  response: ListMigrationsResponse,
} as const satisfies Endpoint;
export type ListMigrations = typeof ListMigrations;
