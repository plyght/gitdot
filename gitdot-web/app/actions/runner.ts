"use server";

import { authorizeDevice } from "@/dal";

export type AuthorizeDeviceActionResult =
  | { success: true }
  | { success: false; error: string };

export async function authorizeDeviceAction(
  userCode: string,
): Promise<AuthorizeDeviceActionResult> {
  if (!userCode) {
    return { success: false, error: "User code is required" };
  }

  const success = await authorizeDevice({ user_code: userCode });
  if (!success) {
    return { success: false, error: "Failed to authorize device" };
  }

  return { success: true };
}
