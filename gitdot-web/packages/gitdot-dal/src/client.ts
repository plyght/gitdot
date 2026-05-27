import { LocalProvider } from "./provider/client";
import type {
  ResourcePromisesType,
  ResourceRequestsType,
  ResourceResultType,
} from "./provider/types";

export * from "./db";
export * from "./hast";
export * from "./language";
export * from "./provider/client";
export * from "./provider/types";
export { default as gitdotLight } from "./themes/gitdot-light";
export { default as vitesseDark } from "./themes/vitesse-dark";
export { createShikiWorker, createSyncWorker } from "./workers";
export type { ShikiRequest, ShikiResponse } from "./workers/shiki";
export type { SyncRequest, SyncResponse } from "./workers/sync";

export function useResources<S>(
  resources: ResourceResultType<S>,
): ResourcePromisesType<S> {
  return raceRequests(
    [LocalProvider.instance],
    resources.requests,
    resources.promises,
  );
}

function raceRequests<S>(
  providers: LocalProvider[],
  requests: ResourceRequestsType<S>,
  promises: ResourcePromisesType<S>,
): ResourcePromisesType<S> {
  const result: Record<string, Promise<unknown>> = {};

  for (const key of Object.keys(requests)) {
    const replayed = providers.map((p) => p.replay(requests)[key]);
    result[key] = racePromises(promises[key as keyof S], ...replayed);
  }

  return result as ResourcePromisesType<S>;
}

function racePromises<T>(
  first: Promise<T>,
  ...rest: Promise<unknown>[]
): Promise<T | null> {
  return new Promise((resolve) => {
    let remaining = 1 + rest.length;
    let resolved = false;

    const all = [first, ...rest] as Promise<T | null | undefined>[];
    for (const p of all) {
      Promise.resolve(p)
        .then((value) => {
          if (resolved) return;
          if (value != null) {
            resolved = true;
            resolve(value);
          } else if (--remaining === 0) {
            resolve(null);
          }
        })
        .catch(() => {
          if (resolved) return;
          if (--remaining === 0) resolve(null);
        });
    }
  });
}
