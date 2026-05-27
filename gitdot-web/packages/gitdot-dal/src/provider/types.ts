import type {
  RepositoryBlobResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { Root } from "hast";

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
  protected owner: string;
  protected repo: string;

  constructor(owner: string, repo: string) {
    this.owner = owner;
    this.repo = repo;
  }

  abstract getPaths(): Promise<RepositoryPathsResource | null>;
  abstract getCommits(): Promise<RepositoryCommitResource[] | null>;
  abstract getBlob(
    path: string,
    ref?: string,
  ): Promise<RepositoryBlobResource | null>;
  abstract getHast(path: string, ref?: string): Promise<Root | null>;
  abstract getCommit(sha: string): Promise<RepositoryCommitResource | null>;
}
