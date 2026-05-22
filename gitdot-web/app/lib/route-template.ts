type Params = Record<string, string | string[] | undefined>;

/**
 * Reconstruct a Next.js route template from a resolved pathname + params.
 *
 *   pathname = "/mikkel/gitdot/issues/42"
 *   params   = { owner: "mikkel", repo: "gitdot", id: "42" }
 *   → "/[owner]/[repo]/issues/[id]"
 *
 * Best-effort: replaces each param value (at segment boundaries) with `[key]`.
 * Wrong if two unrelated segments happen to share a value, but that's
 * vanishingly rare in gitdot's route shapes.
 */
export function inferRouteTemplate(pathname: string, params: Params): string {
  let template = pathname;
  for (const [key, raw] of Object.entries(params)) {
    const values = Array.isArray(raw) ? raw : raw ? [raw] : [];
    for (const v of values) {
      template = template.replace(`/${v}`, `/[${key}]`);
    }
  }
  return template;
}
