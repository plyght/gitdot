import { structuredPatch } from "diff";
import type { Element } from "hast";

const CONTEXT_LINES = 4;
const MERGE_THRESHOLD = 8;

export type DiffPair = [number | null, number | null];
export type DiffHunk = {
  pairs: DiffPair[];
  removedLines: Set<number>;
  addedLines: Set<number>;
};

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
    { context: CONTEXT_LINES },
  );

  const result: DiffHunk[] = [];
  for (const hunk of patch.hunks) {
    let oldLine = hunk.oldStart - 1;
    let newLine = hunk.newStart - 1;
    const pairs: DiffPair[] = [];
    const removedLines = new Set<number>();
    const addedLines = new Set<number>();

    for (const raw of hunk.lines) {
      const marker = raw[0];
      if (marker === "-") {
        pairs.push([oldLine, null]);
        removedLines.add(oldLine);
        oldLine++;
      } else if (marker === "+") {
        pairs.push([null, newLine]);
        addedLines.add(newLine);
        newLine++;
      } else if (marker === " ") {
        pairs.push([oldLine, newLine]);
        oldLine++;
        newLine++;
      }
    }
    result.push({ pairs: alignPairs(pairs), removedLines, addedLines });
  }

  return mergeHunks(result);
}

/**
 * this function does two things:
 * - zip lines (e.g., - a - b + c + d) => pairs a with c and b with d
 * - pad sentinels if line count mismatches (e.g., 3 removals, 4 additions)
 */
export function alignPairs(pairs: DiffPair[]): DiffPair[] {
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

  for (const [L, R] of pairs) {
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
 * regardless of the context window we set, there are unfortunate cases
 *
 * where we will have a small hidden section in between two hunks (e.g,. hiding two lines)
 * this is ugly, so we merge them
 *
 * since hunks are alraedy paired before this, we can just increment the counts in between each hunk
 */
export function mergeHunks(hunks: DiffHunk[]): DiffHunk[] {
  if (hunks.length <= 1) return hunks;
  const result: DiffHunk[] = [hunks[0]];
  for (let i = 1; i < hunks.length; i++) {
    const prev = result[result.length - 1];
    const next = hunks[i];
    const prevLast = prev.pairs.at(-1);
    const nextFirst = next.pairs[0];
    const prevL = prevLast?.[0];
    const prevR = prevLast?.[1];
    const nextL = nextFirst?.[0];

    if (prevL == null || prevR == null || nextL == null) {
      result.push(next);
      continue;
    }

    const gap = nextL - prevL - 1;
    if (gap <= 0 || gap > MERGE_THRESHOLD) {
      result.push(next);
      continue;
    }

    const fill: DiffPair[] = [];
    for (let k = 1; k <= gap; k++) fill.push([prevL + k, prevR + k]);

    result[result.length - 1] = {
      pairs: [...prev.pairs, ...fill, ...next.pairs],
      removedLines: new Set([...prev.removedLines, ...next.removedLines]),
      addedLines: new Set([...prev.addedLines, ...next.addedLines]),
    };
  }
  return result;
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
    for (const [L, R] of hunk.pairs) {
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
