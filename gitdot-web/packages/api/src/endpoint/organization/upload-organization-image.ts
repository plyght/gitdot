import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const UploadOrganizationImage = {
  path: "/organization/{org_name}/image",
  method: "POST",
  request: z.instanceof(Blob),
  response: z.void(),
} as const satisfies Endpoint;
export type UploadOrganizationImage = typeof UploadOrganizationImage;
