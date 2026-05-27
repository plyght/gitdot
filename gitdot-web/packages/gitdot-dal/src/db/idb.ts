"use client";

import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
} from "gitdot-api";
import type { Root } from "hast";
import { type IDBPDatabase, openDB } from "idb";
import type { GitdotDatabase, RepositoryMetadata } from "./types";

const commitKey = (owner: string, repo: string, sha: string) =>
  `${owner}/${repo}/${sha}`;
const repoKey = (owner: string, repo: string) => `${owner}/${repo}`;
const pathKey = (owner: string, repo: string, path: string) =>
  `${owner}/${repo}/${path}`;

let dbPromise: Promise<IDBPDatabase> | null = null;
function getDb(): Promise<IDBPDatabase> {
  if (!dbPromise) {
    dbPromise = openDB("gitdot", 14, {
      upgrade(db, oldVersion) {
        if (oldVersion < 11) {
          if (db.objectStoreNames.contains("blobs"))
            db.deleteObjectStore("blobs");
          if (db.objectStoreNames.contains("hasts"))
            db.deleteObjectStore("hasts");
        }
        if (oldVersion < 12) {
          // dual-theme HASTs: drop entries highlighted before vitesse-dark was added.
          if (db.objectStoreNames.contains("hasts"))
            db.deleteObjectStore("hasts");
        }
        if (oldVersion < 13) {
          // CommitAuthorResource shape change: name (now gitdot slug) + git_name.
          if (db.objectStoreNames.contains("commits"))
            db.deleteObjectStore("commits");
        }
        if (oldVersion < 14) {
          if (db.objectStoreNames.contains("questions"))
            db.deleteObjectStore("questions");
          if (db.objectStoreNames.contains("reviews"))
            db.deleteObjectStore("reviews");
          if (db.objectStoreNames.contains("builds"))
            db.deleteObjectStore("builds");
        }
        if (!db.objectStoreNames.contains("commits"))
          db.createObjectStore("commits");
        if (!db.objectStoreNames.contains("paths"))
          db.createObjectStore("paths");
        if (!db.objectStoreNames.contains("blobs"))
          db.createObjectStore("blobs");
        if (!db.objectStoreNames.contains("hasts"))
          db.createObjectStore("hasts");
        if (db.objectStoreNames.contains("settings"))
          db.deleteObjectStore("settings");
        if (!db.objectStoreNames.contains("metadata"))
          db.createObjectStore("metadata");
      },
    });
  }
  return dbPromise;
}

function withTimings(db: GitdotDatabase): GitdotDatabase {
  return new Proxy(db, {
    get(target, prop, receiver) {
      const value = Reflect.get(target, prop, receiver);
      if (typeof value !== "function") return value;
      return async (...args: unknown[]) => {
        const start = performance.now();
        try {
          return await (value as (...a: unknown[]) => Promise<unknown>).apply(
            receiver,
            args,
          );
        } finally {
          console.log(
            `idb.${String(prop)} ${(performance.now() - start).toFixed(2)}ms`,
          );
        }
      };
    },
  });
}

export function openIdb(): GitdotDatabase {
  if (typeof indexedDB === "undefined") {
    return new Proxy({} as GitdotDatabase, {
      get: () => () => Promise.resolve(null),
    });
  }

  getDb();

  return withTimings({
    async getPaths(owner, repo) {
      const db = await getDb();
      const prefix = `${repoKey(owner, repo)}/`;
      const range = IDBKeyRange.bound(prefix, `${prefix}￿`);
      const rows = await db.getAll("paths", range);
      if (rows.length === 0) return null;
      const { ref_name, commit_sha } = rows[0];
      return {
        ref_name,
        commit_sha,
        entries: rows.map(({ ref_name, commit_sha, ...e }) => e),
      };
    },

    async putPaths(owner, repo, paths) {
      const db = await getDb();
      const { ref_name, commit_sha } = paths;
      const tx = db.transaction("paths", "readwrite");
      await Promise.all([
        ...paths.entries.map((e) =>
          tx.store.put(
            { ref_name, commit_sha, ...e },
            pathKey(owner, repo, e.path),
          ),
        ),
        tx.done,
      ]);
    },

    async getCommit(owner, repo, sha) {
      const db = await getDb();
      return (await db.get("commits", commitKey(owner, repo, sha))) ?? null;
    },

    async getCommits(owner, repo) {
      const db = await getDb();
      const prefix = `${owner}/${repo}/`;
      const range = IDBKeyRange.bound(prefix, `${prefix}￿`);
      return db.getAll("commits", range);
    },

    async putCommit(owner, repo, commit) {
      const db = await getDb();
      await db.put("commits", commit, commitKey(owner, repo, commit.sha));
    },

    async putCommits(owner, repo, commits) {
      const db = await getDb();
      const tx = db.transaction("commits", "readwrite");
      await Promise.all([
        ...commits.map((c) => tx.store.put(c, commitKey(owner, repo, c.sha))),
        tx.done,
      ]);
    },

    async getBlob(owner: string, repo: string, path: string) {
      const db = await getDb();
      return (await db.get("blobs", pathKey(owner, repo, path))) ?? null;
    },

    async putBlob(
      owner: string,
      repo: string,
      path: string,
      blob: RepositoryBlobResource,
    ) {
      const db = await getDb();
      await db.put("blobs", blob, pathKey(owner, repo, path));
    },

    async putBlobs(
      owner: string,
      repo: string,
      blobs: RepositoryBlobsResource,
    ) {
      const db = await getDb();
      const tx = db.transaction("blobs", "readwrite");
      await Promise.all([
        ...blobs.blobs.map((b) =>
          tx.store.put(b, pathKey(owner, repo, b.path)),
        ),
        tx.done,
      ]);
    },

    async getHast(owner: string, repo: string, path: string) {
      const db = await getDb();
      return (await db.get("hasts", pathKey(owner, repo, path))) ?? null;
    },

    async putHast(owner: string, repo: string, path: string, hast: Root) {
      const db = await getDb();
      await db.put("hasts", hast, pathKey(owner, repo, path));
    },

    async getMetadata(owner, repo) {
      const db = await getDb();
      return (await db.get("metadata", repoKey(owner, repo))) ?? null;
    },

    async putMetadata(owner, repo, metadata: RepositoryMetadata) {
      const db = await getDb();
      await db.put("metadata", metadata, repoKey(owner, repo));
    },
  });
}
