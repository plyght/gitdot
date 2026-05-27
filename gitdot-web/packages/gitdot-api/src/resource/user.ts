import { z } from "zod";
import { OrganizationMemberResource } from "./organization";

export const UserResource = z.object({
  id: z.uuid(),
  name: z.string(),
  location: z.string().nullable().optional(),
  readme: z.string().nullable().optional(),
  links: z.array(z.string()).default([]),
  display_name: z.string().nullable().optional(),
  created_at: z.iso.datetime(),
});
export type UserResource = z.infer<typeof UserResource>;

export const UserEmailResource = z.object({
  email: z.string(),
  is_primary: z.boolean(),
  is_verified: z.boolean(),
  created_at: z.iso.datetime(),
});
export type UserEmailResource = z.infer<typeof UserEmailResource>;

export const CurrentUserResource = z.object({
  id: z.uuid(),
  name: z.string(),
  emails: z.array(UserEmailResource),
  memberships: z.array(OrganizationMemberResource),
  location: z.string().nullable().optional(),
  readme: z.string().nullable().optional(),
  links: z.array(z.string()).default([]),
  display_name: z.string().nullable().optional(),
  created_at: z.iso.datetime(),
});
export type CurrentUserResource = z.infer<typeof CurrentUserResource>;
