import { getVercelOidcToken } from "@vercel/oidc";
import { LogWebVitalRequest } from "gitdot-api";
import type { NextRequest } from "next/server";
import { NextResponse } from "next/server";

import { getSession } from "@/lib/auth";

const METRICS_URL = process.env.GITDOT_METRICS_URL ?? "http://localhost:8084";

const drop = () => new NextResponse(null, { status: 204 });

export async function POST(req: NextRequest): Promise<NextResponse> {
  let body: unknown;
  try {
    body = await req.json();
  } catch (err) {
    console.warn("[metrics/web-vital] invalid JSON body", err);
    return drop();
  }

  const enriched = {
    ...(typeof body === "object" && body !== null ? body : { events: [] }),
    country: req.headers.get("x-vercel-ip-country") ?? undefined,
    region: req.headers.get("x-vercel-ip-country-region") ?? undefined,
    city: req.headers.get("x-vercel-ip-city") ?? undefined,
  };
  const parsed = LogWebVitalRequest.safeParse(enriched);
  if (!parsed.success) {
    console.warn("[metrics/web-vital] schema rejected", parsed.error.issues);
    return drop();
  }

  const oidc = await getVercelOidcToken().catch(() => null);
  if (!oidc) return drop();

  const session = await getSession();
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    "X-Vercel-OIDC-Token": oidc,
  };
  if (session) headers.Authorization = `Bearer ${session.access_token}`;

  const res = await fetch(`${METRICS_URL}/metrics/web-vital`, {
    method: "POST",
    headers,
    body: JSON.stringify(parsed.data),
  }).catch((err) => {
    console.warn("[metrics/web-vital] upstream fetch failed", err);
    return null;
  });
  if (res && !res.ok) {
    console.warn(
      `[metrics/web-vital] upstream non-2xx ${res.status} ${res.statusText}`,
    );
  }

  return drop();
}
