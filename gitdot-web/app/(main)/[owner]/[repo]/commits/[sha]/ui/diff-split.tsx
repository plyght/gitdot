import type { Element } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import {
  type DiffHunk,
  expandLines,
  pairLines,
} from "@/(main)/[owner]/[repo]/util";
import { pluralize } from "@/util/string";
import { DiffLine } from "./diff-line";

function hiddenLineCount(prev: DiffHunk, next: DiffHunk): number {
  const prevLine = prev[prev.length - 1]?.lhs?.line_number;
  const nextLine = next[0]?.lhs?.line_number;
  if (prevLine === undefined || nextLine === undefined) return 0;
  return Math.max(0, nextLine - prevLine - 1);
}

export function DiffSplit({
  leftSpans,
  rightSpans,
  hunks,
}: {
  leftSpans: Element[];
  rightSpans: Element[];
  hunks: DiffHunk[];
}) {
  return (
    <div className="flex flex-col w-full">
      {hunks.map((hunk, index) => {
        return (
          <Fragment
            key={`${hunk[0].lhs?.line_number}-${hunk[0].rhs?.line_number}`}
          >
            {index > 0 && (
              <button
                type="button"
                className="flex w-full h-6 items-center justify-center bg-sidebar border-y border-border font-mono text-xs text-muted-foreground hover:text-foreground transition-colors duration-200 cursor-pointer"
              >
                {pluralize(hiddenLineCount(hunks[index - 1], hunk), "line")}...
              </button>
            )}
            <DiffSection
              hunk={hunk}
              leftSpans={leftSpans}
              rightSpans={rightSpans}
            />
          </Fragment>
        );
      })}
    </div>
  );
}

const sentinelSpan: Element = {
  type: "element",
  tagName: "diffline",
  properties: {
    class: "line w-full",
    "data-line-type": "sentinel",
  },
  children: [],
};

function DiffSection({
  hunk,
  leftSpans,
  rightSpans,
}: {
  hunk: DiffHunk;
  leftSpans: Element[];
  rightSpans: Element[];
}) {
  const expandedLines = expandLines(
    pairLines(hunk),
    leftSpans.length,
    rightSpans.length,
  );

  const getSpanOrSentinel = (index: number | null, spans: Element[]) =>
    index !== null && index < spans.length ? spans[index] : sentinelSpan;

  const withSide = (span: Element, side: "old" | "new"): Element => ({
    ...span,
    properties: { ...span.properties, "data-side": side },
  });

  const leftSpansChunk = expandedLines.map(([left]) =>
    withSide(getSpanOrSentinel(left, leftSpans), "old"),
  );
  const rightSpansChunk = expandedLines.map(([, right]) =>
    withSide(getSpanOrSentinel(right, rightSpans), "new"),
  );

  const container: Element = {
    type: "element",
    tagName: "div",
    properties: {
      className: "flex w-full",
    },
    children: [
      {
        type: "element",
        tagName: "pre",
        properties: {
          className:
            "flex flex-col w-1/2 overflow-auto scrollbar-none border-border border-r text-sm font-mono",
        },
        children: leftSpansChunk,
      },
      {
        type: "element",
        tagName: "pre",
        properties: {
          className:
            "flex flex-col w-1/2 overflow-auto scrollbar-none text-sm font-mono",
        },
        children: rightSpansChunk,
      },
    ],
  };

  return toJsxRuntime(container, {
    Fragment,
    jsx,
    jsxs,
    components: {
      diffline: (props) => <DiffLine {...props} />,
    },
  }) as JSX.Element;
}
