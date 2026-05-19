"use client";

import type {
  BuildResource,
  QuestionResource,
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  ReviewResource,
} from "gitdot-api";
import type { Root } from "hast";
import { openIdb } from "@/db";
import { ClientProvider } from "./types";

type Store = {
  paths: RepositoryPathsResource | undefined;
  blobs: RepositoryBlobsResource | undefined;
  commits: RepositoryCommitResource[] | undefined;
  questions: QuestionResource[] | undefined;
  builds: BuildResource[] | undefined;
  blob: Map<string, RepositoryBlobResource>;
  commit: Map<string, RepositoryCommitResource>;
  hast: Map<string, Root>;
  review: Map<number, ReviewResource>;
  build: Map<number, BuildResource>;
};

export class InMemoryProvider extends ClientProvider {
  private store: Store = {
    paths: undefined,
    blobs: undefined,
    commits: undefined,
    questions: undefined,
    builds: undefined,
    blob: new Map(),
    commit: new Map(),
    hast: new Map(),
    review: new Map(),
    build: new Map(),
  };

  async getPaths(): Promise<RepositoryPathsResource | null> {
    return this.store.paths ?? null;
  }

  async getBlob(
    path: string,
    ref?: string,
  ): Promise<RepositoryBlobResource | null> {
    if (ref) return null;
    return this.store.blob.get(path) ?? null;
  }

  async getHast(path: string, ref?: string): Promise<Root | null> {
    if (ref) return null;
    return this.store.hast.get(path) ?? null;
  }

  putHast(path: string, hast: Root): void {
    this.store.hast.set(path, hast);
  }

  async getCommit(sha: string): Promise<RepositoryCommitResource | null> {
    const result = this.store.commit.get(sha) ?? null;
    return result;
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    if (!this.store.commits) return null;
    return this.store.commits.sort(
      (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime(),
    );
  }

  async getCommitFilters(): Promise<RepositoryCommitFilterResource[] | null> {
    return null;
  }

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    return this.store.blobs ?? null;
  }

  async getQuestions(): Promise<QuestionResource[] | null> {
    return this.store.questions ?? null;
  }

  async getReview(number: number): Promise<ReviewResource | null> {
    return this.store.review.get(number) ?? null;
  }

  async getReviews(): Promise<ReviewResource[] | null> {
    if (this.store.review.size === 0) return null;
    return Array.from(this.store.review.values());
  }

  async getBuilds(): Promise<BuildResource[] | null> {
    return this.store.builds ?? null;
  }

  async getBuild(number: number): Promise<BuildResource | null> {
    return this.store.build.get(number) ?? null;
  }

  async initialize(): Promise<void> {
    const t0 = performance.now();
    const time = async <T>(label: string, p: Promise<T>): Promise<T> => {
      const start = performance.now();
      const v = await p;
      const ms = performance.now() - start;
      const size =
        Array.isArray(v) ? v.length
        : v instanceof Map ? v.size
        : v && typeof v === "object" && "blobs" in v && Array.isArray((v as { blobs: unknown[] }).blobs) ? (v as { blobs: unknown[] }).blobs.length
        : v == null ? 0 : 1;
      console.log(`[InMemoryProvider] ${label}: ${ms.toFixed(1)}ms (n=${size})`);
      return v;
    };

    const db = openIdb();
    const metadata = await time("getMetadata", db.getMetadata(this.owner, this.repo));
    if (!metadata) return;
    const { last_commit: commit } = metadata;
    const paths = await time("getPaths", db.getPaths(this.owner, this.repo));
    const blobs = await time("getBlobs", db.getBlobs(this.owner, this.repo, commit));
    const commits = await time("getCommits", db.getCommits(this.owner, this.repo));
    const hasts = await time("getHasts", db.getHasts(this.owner, this.repo, commit));
    const questions = await time("getQuestions", db.getQuestions(this.owner, this.repo));
    const reviews = await time("getReviews", db.getReviews(this.owner, this.repo));
    const builds = await time("getBuilds", db.getBuilds(this.owner, this.repo));

    const tPost = performance.now();
    if (paths) this.store.paths = paths;
    if (blobs) {
      this.store.blobs = blobs;
      for (const b of blobs.blobs) this.store.blob.set(b.path, b);
    }
    if (commits?.length) {
      this.store.commits = commits;
      for (const c of commits) this.store.commit.set(c.sha.slice(0, 7), c);
    }
    if (hasts) this.store.hast = hasts;
    if (questions?.length) this.store.questions = questions;
    if (reviews?.length) {
      for (const r of reviews) this.store.review.set(r.number, r);
    }
    if (builds?.length) {
      this.store.builds = builds;
      for (const b of builds) this.store.build.set(b.number, b);
    }
    console.log(
      `[InMemoryProvider] post-process: ${(performance.now() - tPost).toFixed(1)}ms, total: ${(performance.now() - t0).toFixed(1)}ms`,
    );
  }
}
