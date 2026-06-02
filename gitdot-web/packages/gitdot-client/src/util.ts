import "server-only";

import { getVercelOidcToken } from "@vercel/oidc";
import { headers } from "next/headers";
import type { ZodType } from "zod";
import { getSession } from "./auth";

export const GITDOT_SERVER_URL =
  process.env.GITDOT_SERVER_URL || "http://localhost:8080";

/**
 * Forward the real end-user IP to the backend. Vercel sets `x-forwarded-for` on
 * the incoming request at the edge (clients can't spoof it), so the backend —
 * which would otherwise only see this server's Vercel egress IP — can attribute
 * sessions and rate limits to the actual user.
 */
async function clientIpHeader(): Promise<Record<string, string>> {
  try {
    const h = await headers();
    const ip =
      h.get("x-forwarded-for")?.split(",")[0]?.trim() ?? h.get("x-real-ip");
    return ip ? { "X-Gitdot-Client-Ip": ip } : {};
  } catch {
    return {};
  }
}

export async function authFetch(
  url: string,
  options?: RequestInit,
): Promise<Response> {
  const session = await getSession();
  const oidcToken = await getVercelOidcToken();

  return fetch(url, {
    ...options,
    headers: {
      ...options?.headers,
      ...(session && { Authorization: `Bearer ${session.access_token}` }),
      "X-Vercel-OIDC-Token": oidcToken,
      ...(await clientIpHeader()),
    },
  });
}

export async function authHead(
  url: string,
  options?: RequestInit,
): Promise<Response> {
  return await authFetch(url, {
    ...options,
    method: "HEAD",
  });
}

export async function authPost(
  url: string,
  request: unknown,
  extraHeaders?: Record<string, string>,
): Promise<Response> {
  return await authFetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...extraHeaders,
    },
    body: JSON.stringify(request),
  });
}

export async function authDelete(
  url: string,
  options?: RequestInit,
): Promise<Response> {
  return await authFetch(url, {
    ...options,
    method: "DELETE",
  });
}

export async function authPatch(
  url: string,
  request: unknown,
): Promise<Response> {
  return await authFetch(url, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  });
}

export class ApiError extends Error {
  constructor(
    public readonly status: number,
    message: string,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

export async function apiErrorFromResponse(
  response: Response,
): Promise<ApiError> {
  let message = response.statusText;
  try {
    const body = await response.json();
    if (typeof body?.message === "string") message = body.message;
  } catch {
    // ignore parse failure, keep statusText
  }
  console.error(`${response.url} failed:`, response.status, message);
  return new ApiError(response.status, message);
}

export async function handleResponse<T>(
  response: Response,
  schema: ZodType<T>,
): Promise<T | null> {
  if (response.status === 404) return null;
  if (response.status === 304) return null;

  if (!response.ok) throw await apiErrorFromResponse(response);

  const data = await response.json();
  return schema.parse(data);
}

export async function handleEmptyResponse(response: Response): Promise<void> {
  if (!response.ok) throw await apiErrorFromResponse(response);
}

/**
 * helper to serialize objects that have non-string values into url parameter queries
 */
export function toQueryString(
  params:
    | Record<string, string | number | boolean | undefined | null>
    | undefined,
): string {
  if (!params) {
    return "";
  }

  const stringParams = Object.fromEntries(
    Object.entries(params)
      .filter(([_, value]) => value !== undefined && value !== null)
      .map(([key, value]) => [key, String(value)]),
  );
  return new URLSearchParams(stringParams).toString();
}
