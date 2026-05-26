import { structuredPatch } from "diff";
import type { Element } from "hast";

export type DiffPair = [number | null, number | null];
export type DiffHunk = DiffPair[];

/**
 * jsdiff's structuredPath generally returns comparable output to unified patch algorithms
 *
 * note that context lines here will show up and down even for an addition only or removal change
 */
export function diffFiles(
  leftContent: string,
  rightContent: string,
): DiffHunk[] {
  const patch = structuredPatch(
    "left",
    "right",
    leftContent,
    rightContent,
    undefined,
    undefined,
    { context: 4 },
  );

  const result: DiffHunk[] = [];
  for (const hunk of patch.hunks) {
    let oldLine = hunk.oldStart - 1;
    let newLine = hunk.newStart - 1;
    const pairs: DiffPair[] = [];

    for (const raw of hunk.lines) {
      const marker = raw[0];
      if (marker === "-") {
        pairs.push([oldLine, null]);
        oldLine++;
      } else if (marker === "+") {
        pairs.push([null, newLine]);
        newLine++;
      } else if (marker === " ") {
        pairs.push([oldLine, newLine]);
        oldLine++;
        newLine++;
      }
    }
    result.push(pairs);
  }

  return result;
}

/**
 * this function does two things:
 * - zip lines (e.g., - a - b + c + d) => pairs a with c and b with d
 * - pad sentinels if line count mismatches (e.g., 3 removals, 4 additions)
 */
export function pairLines(hunk: DiffHunk): DiffPair[] {
  const result: DiffPair[] = [];
  let pendingLhs: number[] = [];
  let pendingRhs: number[] = [];

  const flush = () => {
    const n = Math.max(pendingLhs.length, pendingRhs.length);
    for (let i = 0; i < n; i++) {
      result.push([
        i < pendingLhs.length ? pendingLhs[i] : null,
        i < pendingRhs.length ? pendingRhs[i] : null,
      ]);
    }
    pendingLhs = [];
    pendingRhs = [];
  };

  for (const [L, R] of hunk) {
    if (L !== null && R !== null) {
      flush();
      result.push([L, R]);
    } else if (L !== null) {
      pendingLhs.push(L);
    } else if (R !== null) {
      pendingRhs.push(R);
    }
  }
  flush();

  return result;
}

/**
 * collects the set of changed line numbers per side
 *
 * used by renderSpans to tag added/removed lines for whole-line background
 * coloring. context (both-sided anchor) pairs are not changes and are excluded.
 */
export function getChangedLines(hunks: DiffHunk[]): {
  leftLines: Set<number>;
  rightLines: Set<number>;
} {
  const leftLines = new Set<number>();
  const rightLines = new Set<number>();
  for (const hunk of hunks) {
    for (const [L, R] of hunk) {
      if (L !== null && R === null) leftLines.add(L);
      if (R !== null && L === null) rightLines.add(R);
    }
  }
  return { leftLines, rightLines };
}

// ============================================================================
// unified vs split diff view heuristics
// ============================================================================

const SPLIT_MAX_LINE_LENGTH = 80;
const SPLIT_MIN_MATCH_RATIO = 0.25;
const UNIFIED_MAX_PAIRS_COUNT = 50;

export function preferSplit(
  leftSpans: Element[],
  rightSpans: Element[],
  hunks: DiffHunk[],
): boolean {
  let maxLen = 0;
  let matched = 0;
  let total = 0;

  for (const hunk of hunks) {
    for (const [L, R] of hunk) {
      if (L !== null) {
        const span = leftSpans[L];
        if (span) maxLen = Math.max(maxLen, spanTextLength(span));
      }
      if (R !== null) {
        const span = rightSpans[R];
        if (span) maxLen = Math.max(maxLen, spanTextLength(span));
      }

      // TODO: with context anchors now in every hunk, this match ratio is
      // inflated; count only non-anchor pairs against `total` if re-enabled.
      if (L !== null && R !== null) matched++;
      total++;
    }
  }

  if (total === 0) return false;
  if (total > UNIFIED_MAX_PAIRS_COUNT) return true;

  return (
    maxLen <= SPLIT_MAX_LINE_LENGTH && matched / total >= SPLIT_MIN_MATCH_RATIO
  );
}

function spanTextLength(span: Element): number {
  let len = 0;
  for (const child of span.children) {
    if (child.type === "text") len += child.value.length;
    else if (child.type === "element") len += spanTextLength(child);
  }
  return len;
}
