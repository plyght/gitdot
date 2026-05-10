"use server";

import type { OrganizationResource } from "gitdot-api";
import { refresh } from "next/cache";
import {
  createOrganization,
  uploadOrganizationImage,
} from "@/dal/organization";

export type CreateOrganizationActionResult =
  | { organization: OrganizationResource }
  | { error: string };

export async function createOrganizationAction(
  formData: FormData,
): Promise<CreateOrganizationActionResult> {
  const name = formData.get("org-name") as string;
  const readmeRaw = formData.get("org-readme") as string | null;
  const readme = (readmeRaw ?? "").trim() || undefined;

  if (!name) {
    return { error: "Organization name is required" };
  }

  const result = await createOrganization(name, readme);
  if (!result) {
    return { error: "Failed to create organization" };
  }

  refresh();
  return { organization: result };
}

export async function uploadOrganizationImageAction(
  orgName: string,
  file: File,
): Promise<{ success: true } | { error: string }> {
  if (!file || file.size === 0) {
    return { error: "No file provided" };
  } else if (file.size > 5 * 1024 * 1024) {
    return { error: "Image must be under 5 MB." };
  } else if (!["image/jpeg", "image/png", "image/webp"].includes(file.type)) {
    return { error: "Unsupported image type — use JPEG, PNG, or WebP." };
  }

  try {
    const ok = await uploadOrganizationImage(orgName, file);
    if (!ok) return { error: "Upload failed — please try again." };
    return { success: true };
  } catch (e) {
    console.error("uploadOrganizationImageAction failed:", e);
    const msg = e instanceof Error ? e.message : "Unknown error";
    return { error: `Upload failed: ${msg}` };
  }
}
