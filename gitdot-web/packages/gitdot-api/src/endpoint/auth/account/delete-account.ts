import { z } from "zod";
import type { Endpoint } from "../../endpoint";

export const DeleteAccountRequest = z.object({});
export type DeleteAccountRequest = z.infer<typeof DeleteAccountRequest>;

export const DeleteAccountResponse = z.void();
export type DeleteAccountResponse = z.infer<typeof DeleteAccountResponse>;

export const DeleteAccount = {
  path: "/auth/account",
  method: "DELETE",
  request: DeleteAccountRequest,
  response: DeleteAccountResponse,
} as const satisfies Endpoint;
export type DeleteAccount = typeof DeleteAccount;
