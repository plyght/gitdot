import type { RepositoryBlobResource } from "gitdot-api";
import type { Element } from "hast";

export type DiffPair = [number | null, number | null];
export type DiffHunk = {
  pairs: DiffPair[];
  removedLines: Set<number>;
  addedLines: Set<number>;
};

export type DiffSpans =
  | {
      kind: "split";
      leftSpans: Element[];
      rightSpans: Element[];
      hunks: DiffHunk[];
    }
  | {
      kind: "unilateral";
      spans: Element[];
      hunks: DiffHunk[];
      side: "left" | "right";
    }
  | { kind: "created"; spans: Element[] }
  | { kind: "deleted" }
  | { kind: "no-change" };

export type DiffEntry = {
  path: string;
  old: RepositoryBlobResource | null;
  new: RepositoryBlobResource | null;
  spans: DiffSpans;
  linesAdded: number;
  linesRemoved: number;
};

export type DiffData = DiffEntry[];
