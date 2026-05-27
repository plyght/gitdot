"use client";

import type {
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { Root } from "hast";
import { openIdb } from "../db";
import { createShikiWorker, createSyncWorker } from "../workers";
import type { ShikiRequest, ShikiResponse } from "../workers/shiki";
import type { SyncResponse } from "../workers/sync";
import { GitdotProvider } from "./types";

export class ClientProvider extends GitdotProvider {
  private static _instance: ClientProvider | null = null;
  static get instance(): ClientProvider {
    if (!ClientProvider._instance) {
      ClientProvider._instance = new ClientProvider();
    }
    return ClientProvider._instance;
  }

  private db = openIdb();
  private paths = new Map<string, RepositoryPathsResource>();
  private commits = new Map<string, RepositoryCommitResource[]>();
  private hasts = new Map<string, Root>();

  private syncWorker: SharedWorker | null = null;
  private shikiWorker: SharedWorker | null = null;
  private syncRequests = new Map<string, () => void>();
  private shikiRequests = new Map<string, (hast: Root) => void>();

  private constructor() {
    super();
    if (typeof SharedWorker === "undefined") return;

    this.syncWorker = createSyncWorker();
    this.syncWorker.port.start();
    this.syncWorker.port.onmessage = (e: MessageEvent<SyncResponse>) => {
      const resolve = this.syncRequests.get(e.data.id);
      if (!resolve) return;
      this.syncRequests.delete(e.data.id);
      resolve();
    };

    this.shikiWorker = createShikiWorker();
    this.shikiWorker.port.start();
    this.shikiWorker.port.onmessage = (e: MessageEvent<ShikiResponse>) => {
      const resolve = this.shikiRequests.get(e.data.id);
      if (!resolve) return;
      this.shikiRequests.delete(e.data.id);
      resolve(e.data.hast);
    };
  }

  syncRepo(owner: string, repo: string): Promise<void> {
    let resolve!: () => void;
    const done = new Promise<void>((r) => {
      resolve = r;
    });
    const id = crypto.randomUUID();
    this.syncRequests.set(id, resolve);
    this.syncWorker?.port.postMessage({ id, owner, repo });
    done.then(() => {
      this.getPaths(owner, repo);
      this.getCommits(owner, repo);
    });
    return done;
  }

  async getPaths(owner: string, repo: string) {
    const key = `${owner}/${repo}`;
    const cached = this.paths.get(key);
    if (cached) return cached;
    const paths = await this.db.getPaths(owner, repo);
    if (paths) this.paths.set(key, paths);
    return paths;
  }

  async getCommits(
    owner: string,
    repo: string,
  ): Promise<RepositoryCommitResource[] | null> {
    const key = `${owner}/${repo}`;
    const cached = this.commits.get(key);
    if (cached) return cached;
    const commits = await this.db.getCommits(owner, repo);
    if (commits === null || commits.length === 0) return null;
    const sorted = commits.sort(
      (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime(),
    );
    this.commits.set(key, sorted);
    return sorted;
  }

  async getBlob(owner: string, repo: string, path: string, _ref?: string) {
    return this.db.getBlob(owner, repo, path);
  }

  async getHast(
    owner: string,
    repo: string,
    path: string,
    _ref?: string,
  ): Promise<Root | null> {
    const key = `${owner}/${repo}/${path}`;
    const cached = this.hasts.get(key);
    if (cached) return cached;

    const blob = await this.db.getBlob(owner, repo, path);
    if (!blob || blob.type !== "file") return null;

    const id = crypto.randomUUID();
    const hast = await new Promise<Root>((resolve) => {
      this.shikiRequests.set(id, resolve);
      this.shikiWorker?.port.postMessage({
        id,
        path,
        content: blob.content,
      } satisfies ShikiRequest);
    });
    this.hasts.set(key, hast);
    return hast;
  }

  async getCommit(owner: string, repo: string, sha: string) {
    const cached = this.commits.get(`${owner}/${repo}`);
    const hit = cached?.find((c) => c.sha === sha || c.sha.startsWith(sha));
    if (hit) return hit;
    return this.db.getCommit(owner, repo, sha);
  }
}
