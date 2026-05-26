import { type DiffHunk, type DiffPair, diffFiles, pairLines } from "../diff";

/**
 * Helper to build a DiffHunk from a readable shorthand.
 * - { lhs: n } for left-only (removed)
 * - { rhs: n } for right-only (added)
 * - { lhs: l, rhs: r } for both-sided (context anchor)
 */
function chunk(
  entries: Array<
    { lhs: number } | { rhs: number } | { lhs: number; rhs: number }
  >,
): DiffHunk {
  return entries.map((entry) => [
    "lhs" in entry ? entry.lhs : null,
    "rhs" in entry ? entry.rhs : null,
  ]);
}

describe("diffFiles", () => {
  test("preserves intra-hunk context lines as anchors in original order", () => {
    // hunk:
    //   - alpha          (line 0 in left)
    //   + gamma          (line 0 in right)
    //     common_one     (line 1 in both)  <-- context anchor
    //     common_two     (line 2 in both)  <-- context anchor
    //     common_three   (line 3 in both)  <-- context anchor
    //   - beta           (line 4 in left)
    const left = `alpha\ncommon_one\ncommon_two\ncommon_three\nbeta\n`;
    const right = `gamma\ncommon_one\ncommon_two\ncommon_three\n`;

    const hunks = diffFiles(left, right);
    expect(hunks).toHaveLength(1);
    expect(hunks[0]).toEqual([
      [0, null],
      [null, 0],
      [1, 1],
      [2, 2],
      [3, 3],
      [4, null],
    ]);
  });

  test("identical content produces no hunks", () => {
    const identical = "a\nb\nc\nd\n";
    expect(diffFiles(identical, identical)).toEqual([]);
  });
});

describe("pairLines", () => {
  const cases: Array<{ name: string; input: DiffHunk; expected: DiffPair[] }> =
    [
      {
        name: "lhs-only block: each removed becomes [lhs, null]",
        input: chunk([{ lhs: 1 }, { lhs: 2 }, { lhs: 3 }]),
        expected: [
          [1, null],
          [2, null],
          [3, null],
        ],
      },
      {
        name: "rhs-only block: each added becomes [null, rhs]",
        input: chunk([{ rhs: 1 }, { rhs: 2 }, { rhs: 3 }]),
        expected: [
          [null, 1],
          [null, 2],
          [null, 3],
        ],
      },
      {
        name: "balanced change block zips removed and added side-by-side",
        input: chunk([{ lhs: 5 }, { lhs: 6 }, { rhs: 5 }, { rhs: 6 }]),
        expected: [
          [5, 5],
          [6, 6],
        ],
      },
      {
        name: "asymmetric block, removed > added: pads right with sentinels",
        input: chunk([{ lhs: 5 }, { lhs: 6 }, { rhs: 5 }]),
        expected: [
          [5, 5],
          [6, null],
        ],
      },
      {
        name: "asymmetric block, added > removed: pads left with sentinels",
        input: chunk([{ lhs: 5 }, { rhs: 5 }, { rhs: 6 }]),
        expected: [
          [5, 5],
          [null, 6],
        ],
      },
      {
        name: "user's mixed hunk: context anchors flush surrounding change blocks",
        input: chunk([
          { lhs: 0 },
          { rhs: 0 },
          { lhs: 1, rhs: 1 },
          { lhs: 2, rhs: 2 },
          { lhs: 3 },
        ]),
        expected: [
          [0, 0],
          [1, 1],
          [2, 2],
          [3, null],
        ],
      },
      {
        name: "two asymmetric blocks separated by context: each flush pads independently",
        input: chunk([
          { lhs: 0 },
          { lhs: 1 },
          { rhs: 0 },
          { lhs: 2, rhs: 1 },
          { lhs: 3, rhs: 2 },
          { lhs: 4 },
          { rhs: 3 },
          { rhs: 4 },
        ]),
        expected: [
          [0, 0],
          [1, null],
          [2, 1],
          [3, 2],
          [4, 3],
          [null, 4],
        ],
      },
    ];

  test.each(cases)("$name", ({ input, expected }) => {
    expect(pairLines(input)).toEqual(expected);
  });

  test("left and right columns always have equal row count", () => {
    for (const { input } of cases) {
      const result = pairLines(input);
      const leftRows = result.length;
      const rightRows = result.length;
      expect(leftRows).toEqual(rightRows);
    }
  });
});
