"use server";

import type { RepositoryDiffFileResource } from "gitdot-api";
import type { Element } from "hast";
import {
  createChangeMaps,
  type DiffHunk,
  diffFiles,
  fileToHast,
  inferLanguage,
  renderSpans,
} from "@/(main)/[owner]/[repo]/util";
import {
  getRepositoryBlobDiffs,
  getRepositoryCommitDiff,
  getReviewDiff,
} from "@/dal";

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
  resource: RepositoryDiffFileResource;
  spans: DiffSpans;
};

export async function renderBlobDiffsAction(
  owner: string,
  repo: string,
  commitShas: string[],
  path: string,
): Promise<Record<string, DiffEntry>> {
  const result = await getRepositoryBlobDiffs(owner, repo, {
    commit_shas: commitShas,
    path,
  });
  if (!result) return {};
  const entries = await Promise.all(
    Object.entries(result.diffs).map(
      async ([sha, file]) =>
        [sha, { resource: file, spans: await renderDiff(file) }] as const,
    ),
  );
  return Object.fromEntries(entries);
}

export async function renderCommitDiffAction(
  owner: string,
  repo: string,
  sha: string,
): Promise<DiffEntry[]> {
  const result = await getRepositoryCommitDiff(owner, repo, sha);
  if (!result) return [];
  return renderDiffs(result.files);
}

export async function renderReviewDiffAction(
  owner: string,
  repo: string,
  number: number | string,
  position: number,
  revision?: number,
  compareTo?: number,
): Promise<DiffEntry[]> {
  const result = await getReviewDiff(
    owner,
    repo,
    number,
    position,
    revision,
    compareTo,
  );
  if (!result) return [];
  return renderDiffs(result.files);
}

async function renderDiffs(
  files: RepositoryDiffFileResource[],
): Promise<DiffEntry[]> {
  const datas = await Promise.all(files.map(renderDiff));
  return files.map((file, i) => ({ resource: file, spans: datas[i] }));
}

async function renderDiff(
  file: RepositoryDiffFileResource,
): Promise<DiffSpans> {
  const left = file.left_content ?? null;
  const right = file.right_content ?? null;
  const lang = inferLanguage(file.path);

  if (left != null && right != null) {
    const hunks = diffFiles(left, right);
    if (hunks.length === 0) return { kind: "no-change" };

    // anchor pairs (both lhs and rhs present) are context, not changes;
    // a hunk is all-additions iff no pair is lhs-only, all-removals iff no pair is rhs-only
    const isAllAdditions = hunks.every((h) =>
      h.every((p) => !(p.lhs && !p.rhs)),
    );
    const isAllRemovals = hunks.every((h) =>
      h.every((p) => !(p.rhs && !p.lhs)),
    );
    const { leftLines, rightLines } = createChangeMaps(hunks);

    if (isAllAdditions || isAllRemovals) {
      const side = isAllAdditions ? "right" : "left";
      const content = isAllAdditions ? right : left;
      const changedLines = isAllAdditions ? rightLines : leftLines;
      const spans = await renderSpans(
        side,
        content,
        lang,
        changedLines,
        "vitesse",
      );
      return { kind: "unilateral", spans, hunks, side };
    }

    const [leftSpans, rightSpans] = await Promise.all([
      renderSpans("left", left, lang, leftLines),
      renderSpans("right", right, lang, rightLines),
    ]);
    return { kind: "split", leftSpans, rightSpans, hunks };
  }

  if (left != null) return { kind: "deleted" };
  if (right != null) {
    const hast = await fileToHast(right, lang, "vitesse", [
      {
        line(node, lineNumber) {
          node.type = "element";
          node.tagName = "diffline";
          node.properties["data-line-number"] = lineNumber;
          node.properties["data-line-type"] = "added";
        },
      },
    ]);
    const pre = hast.children[0] as Element;
    const code = pre.children[0] as Element;
    const spans = code.children.filter(
      (child): child is Element => child.type === "element",
    );
    return { kind: "created", spans };
  }
  return { kind: "no-change" };
}
