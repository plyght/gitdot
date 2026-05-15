import { z } from "zod";
import { OrganizationMemberResource } from "./organization";

export const UserResource = z.object({
  id: z.uuid(),
  name: z.string(),
  email: z.string(),
  created_at: z.iso.datetime(),
  location: z.string().nullable().optional(),
  readme: z.string().nullable().optional(),
  links: z.array(z.string()).default([]),
  display_name: z.string().nullable().optional(),
});
export type UserResource = z.infer<typeof UserResource>;

export const CurrentUserResource = z.object({
  user: UserResource,
  memberships: z.array(OrganizationMemberResource),
});
export type CurrentUserResource = z.infer<typeof CurrentUserResource>;
