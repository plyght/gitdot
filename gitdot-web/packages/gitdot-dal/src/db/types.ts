import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";

export type RepositoryMetadata = {
  last_commit: string;
  last_updated: string;
};

export interface GitdotDatabase {
  getPaths(
    owner: string,
    repo: string,
  ): Promise<RepositoryPathsResource | null>;

  putPaths(
    owner: string,
    repo: string,
    paths: RepositoryPathsResource,
  ): Promise<void>;

  getCommit(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<RepositoryCommitResource | null>;

  getCommits(owner: string, repo: string): Promise<RepositoryCommitResource[]>;

  putCommit(
    owner: string,
    repo: string,
    commit: RepositoryCommitResource,
  ): Promise<void>;

  putCommits(
    owner: string,
    repo: string,
    commits: RepositoryCommitResource[],
  ): Promise<void>;

  clear(): Promise<void>;

  putBlob(
    owner: string,
    repo: string,
    path: string,
    blob: RepositoryBlobResource,
  ): Promise<void>;

  getBlob(
    owner: string,
    repo: string,
    path: string,
  ): Promise<RepositoryBlobResource | null>;

  putBlobs(
    owner: string,
    repo: string,
    blobs: RepositoryBlobsResource,
  ): Promise<void>;

  getMetadata(owner: string, repo: string): Promise<RepositoryMetadata | null>;

  putMetadata(
    owner: string,
    repo: string,
    metadata: RepositoryMetadata,
  ): Promise<void>;
}
