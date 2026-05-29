import type {
  RepositoryBlobResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { Root } from "hast";
import type { DiffData } from "../diff/types";

export type ResourceDefinition = Record<
  string,
  (provider: GitdotProvider) => Promise<unknown>
>;

export type ResourceRequestType = { method: string; args: unknown[] };
export type ResourceRequestsType<S> = {
  [K in keyof S]: ResourceRequestType;
};

export type ResourcePromisesType<S> = {
  [K in keyof S]: Promise<S[K]>;
};

export type ResourceResultType<S> = {
  promises: ResourcePromisesType<S>;
  requests: ResourceRequestsType<S>;
};

export type ShapeFromDefinition<T extends ResourceDefinition> = {
  [K in keyof T]: Awaited<ReturnType<T[K]>>;
};

export abstract class GitdotProvider {
  abstract getPaths(
    owner: string,
    repo: string,
  ): Promise<RepositoryPathsResource | null>;
  abstract getCommits(
    owner: string,
    repo: string,
  ): Promise<RepositoryCommitResource[] | null>;
  abstract getBlob(
    owner: string,
    repo: string,
    path: string,
    ref?: string,
  ): Promise<RepositoryBlobResource | null>;
  abstract getHast(
    owner: string,
    repo: string,
    path: string,
    ref?: string,
  ): Promise<Root | null>;
  abstract getCommit(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<RepositoryCommitResource | null>;
  abstract getCommitDiff(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<DiffData>;
}
