import "server-only";

import type {
  BuildResource,
  QuestionResource,
  RepositoryBlobResource,
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  ReviewResource,
} from "gitdot-api";
import {
  getBuild as dalGetBuild,
  getBuilds as dalGetBuilds,
  getReview as dalGetReview,
  getRepositoryBlob,
  getRepositoryCommit,
  getRepositoryPaths,
  listQuestions,
  listRepositoryCommitFilters,
  listRepositoryCommits,
  listReviews,
} from "gitdot-client";
import type { Root } from "hast";
import { fileToHast } from "../hast";
import { inferLanguage } from "../language";
import {
  GitdotProvider,
  type ResourceDefinition,
  type ResourceRequestType,
  type ResourceResultType,
  type ShapeFromDefinition,
} from "./types";

export class ServerProvider extends GitdotProvider {
  fetch<T extends ResourceDefinition>(
    def: T,
  ): ResourceResultType<ShapeFromDefinition<T>> {
    const promises: Record<string, Promise<unknown>> = {};
    const requests: Record<string, ResourceRequestType> = {};

    for (const [key, factory] of Object.entries(def)) {
      let request: ResourceRequestType | null = null;

      const proxy = new Proxy(this, {
        get(target, prop: string) {
          const func = target[prop as keyof typeof target];
          if (typeof func !== "function") {
            throw new Error("Provider.fetch should only invoke methods");
          } else if (request) {
            throw new Error(
              "Provider.fetch should only invoke a single method",
            );
          }

          return (...args: unknown[]) => {
            request = { method: prop, args };
            return func.apply(target, args);
          };
        },
      });

      const promise = factory(proxy);
      if (!request) {
        throw new Error("Provider.fetch did not invoke any methods");
      }

      promises[key] = promise;
      requests[key] = request;
    }

    return { promises, requests } as ResourceResultType<ShapeFromDefinition<T>>;
  }

  async getPaths(): Promise<RepositoryPathsResource | null> {
    return await getRepositoryPaths(this.owner, this.repo);
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    const result = await listRepositoryCommits(this.owner, this.repo);
    return result?.data ?? null;
  }

  async getBlob(
    path: string,
    ref?: string,
  ): Promise<RepositoryBlobResource | null> {
    return await getRepositoryBlob(this.owner, this.repo, {
      path,
      ...(ref && { ref_name: ref }),
    });
  }

  async getHast(path: string, ref?: string): Promise<Root | null> {
    const blob = await getRepositoryBlob(this.owner, this.repo, {
      path,
      ...(ref && { ref_name: ref }),
    });
    if (!blob || blob.type === "folder") return null;
    const lang = inferLanguage(path);
    return fileToHast(blob.content, lang, "vitesse", []);
  }

  async getCommit(sha: string): Promise<RepositoryCommitResource | null> {
    return await getRepositoryCommit(this.owner, this.repo, sha);
  }

  async getCommitFilters(): Promise<RepositoryCommitFilterResource[] | null> {
    const result = await listRepositoryCommitFilters(this.owner, this.repo);
    return result?.data ?? null;
  }

  async getQuestions(): Promise<QuestionResource[] | null> {
    const result = await listQuestions(this.owner, this.repo);
    return result?.data ?? null;
  }

  async getReview(number: number): Promise<ReviewResource | null> {
    return await dalGetReview(this.owner, this.repo, number);
  }

  async getReviews(): Promise<ReviewResource[] | null> {
    const result = await listReviews(this.owner, this.repo);
    return result?.data ?? null;
  }

  async getBuilds(): Promise<BuildResource[] | null> {
    const result = await dalGetBuilds(this.owner, this.repo);
    return result?.data ?? null;
  }

  async getBuild(number: number): Promise<BuildResource | null> {
    return await dalGetBuild(this.owner, this.repo, number);
  }
}
