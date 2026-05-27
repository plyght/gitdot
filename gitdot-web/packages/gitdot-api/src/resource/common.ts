import { z } from "zod";

/**
 * Build a Zod schema for a paginated response of `item`s.
 *
 * Shape: `{ data: [...], next_cursor?: "..." }`. `next_cursor` is omitted
 * by the server on the final page; clients pass it back as `?cursor=...`
 * to fetch the next page.
 */
export const page = <T extends z.ZodTypeAny>(item: T) =>
  z.object({
    data: z.array(item),
    next_cursor: z.string().optional(),
  });
