import "server-only";

import { ServerProvider } from "./provider/server";
import type { ResourceDefinition } from "./provider/types";

export * from "./hast";
export * from "./language";
export * from "./provider/server";
export * from "./provider/types";

export function fetchResources<T extends ResourceDefinition>(
  owner: string,
  repo: string,
  resources: T,
) {
  return new ServerProvider(owner, repo).fetch(resources);
}
