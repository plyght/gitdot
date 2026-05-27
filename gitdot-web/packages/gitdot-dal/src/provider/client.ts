"use client";

import type {
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { Root } from "hast";
import { openIdb } from "../db";
import { GitdotProvider, type ResourceRequestType } from "./types";

export class LocalProvider extends GitdotProvider {
  private static _instance: LocalProvider | null = null;
  static get instance(): LocalProvider {
    if (!LocalProvider._instance) {
      LocalProvider._instance = new LocalProvider();
    }
    return LocalProvider._instance;
  }

  private db = openIdb();
  private paths = new Map<string, RepositoryPathsResource>();
  private commits = new Map<string, RepositoryCommitResource[]>();

  replay(
    requests: Record<string, ResourceRequestType>,
  ): Record<string, Promise<unknown>> {
    const results: Record<string, Promise<unknown>> = {};
    for (const [key, { method, args }] of Object.entries(requests)) {
      const func = this[method as keyof this];
      if (typeof func !== "function") {
        throw new Error(`LocalProvider has no method "${method}"`);
      }
      results[key] = func.apply(this, args);
    }
    return results;
  }

  async initialize(owner: string, repo: string) {
    await Promise.all([
      this.getPaths(owner, repo),
      this.getCommits(owner, repo),
    ]);
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
    return this.db.getHast(owner, repo, path);
  }

  async getCommit(owner: string, repo: string, sha: string) {
    const cached = this.commits.get(`${owner}/${repo}`);
    const hit = cached?.find((c) => c.sha === sha || c.sha.startsWith(sha));
    if (hit) return hit;
    return this.db.getCommit(owner, repo, sha);
  }
}
