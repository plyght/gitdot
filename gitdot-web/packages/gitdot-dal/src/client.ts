import { ClientProvider } from "./provider/client";
import type {
  ResourcePromisesType,
  ResourceResultType,
} from "./provider/types";

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
  const localPromises = ClientProvider.instance.replay(resources.requests);
  const keys = Object.keys(resources.requests);
  const start = performance.now();
  const timings: Record<
    string,
    { client: number | null; server: number | null }
  > = {};
  for (const key of keys) timings[key] = { client: null, server: null };

  let remaining = keys.length * 2;
  const tick = () => {
    if (--remaining === 0) console.log("[useResources]", timings);
  };

  const result: Record<string, Promise<unknown>> = {};
  for (const key of keys) {
    const serverPromise = resources.promises[key as keyof S];
    const clientPromise = localPromises[key];
    Promise.resolve(clientPromise).finally(() => {
      timings[key].client = +(performance.now() - start).toFixed(2);
      tick();
    });
    Promise.resolve(serverPromise).finally(() => {
      timings[key].server = +(performance.now() - start).toFixed(2);
      tick();
    });
    result[key] = racePromises(serverPromise, clientPromise);
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
