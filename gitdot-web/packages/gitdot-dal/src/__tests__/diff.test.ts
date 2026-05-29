import { diffFiles } from "../diff/algo";

describe("diffFiles", () => {
  test("returns already-paired pairs plus per-side change sets", () => {
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
    expect(hunks[0].pairs).toEqual([
      [0, 0],
      [1, 1],
      [2, 2],
      [3, 3],
      [4, null],
    ]);
    expect(hunks[0].removedLines).toEqual(new Set([0, 4]));
    expect(hunks[0].addedLines).toEqual(new Set([0]));
  });

  test("identical content produces no hunks", () => {
    const identical = "a\nb\nc\nd\n";
    expect(diffFiles(identical, identical)).toEqual([]);
  });

  test("zips consecutive removals and additions into pairs", () => {
    // - a / - b / + w / + x emitted by jsdiff as removals-then-additions;
    // alignPairs zips them back into aligned [old, new] pairs.
    const left = `a\nb\nc\nd\n`;
    const right = `w\nx\nc\nd\n`;

    const hunks = diffFiles(left, right);
    expect(hunks).toHaveLength(1);
    expect(hunks[0].pairs).toEqual([
      [0, 0],
      [1, 1],
      [2, 2],
      [3, 3],
    ]);
    expect(hunks[0].removedLines).toEqual(new Set([0, 1]));
    expect(hunks[0].addedLines).toEqual(new Set([0, 1]));
  });

  test("pads with sentinels when removal/addition counts differ", () => {
    // 3 removals vs 1 addition: extra removals pair against null.
    const left = `a\nb\nc\nkeep\n`;
    const right = `x\nkeep\n`;

    const hunks = diffFiles(left, right);
    expect(hunks).toHaveLength(1);
    expect(hunks[0].pairs).toEqual([
      [0, 0],
      [1, null],
      [2, null],
      [3, 1],
    ]);
    expect(hunks[0].removedLines).toEqual(new Set([0, 1, 2]));
    expect(hunks[0].addedLines).toEqual(new Set([0]));
  });

  test("pure insertion adds a line on the right only", () => {
    const left = `a\nb\nc\n`;
    const right = `a\nNEW\nb\nc\n`;

    const hunks = diffFiles(left, right);
    expect(hunks).toHaveLength(1);
    expect(hunks[0].pairs).toEqual([
      [0, 0],
      [null, 1],
      [1, 2],
      [2, 3],
    ]);
    expect(hunks[0].removedLines).toEqual(new Set());
    expect(hunks[0].addedLines).toEqual(new Set([1]));
  });

  test("pure deletion removes a line on the left only", () => {
    const left = `a\nGONE\nb\nc\n`;
    const right = `a\nb\nc\n`;

    const hunks = diffFiles(left, right);
    expect(hunks).toHaveLength(1);
    expect(hunks[0].pairs).toEqual([
      [0, 0],
      [1, null],
      [2, 1],
      [3, 2],
    ]);
    expect(hunks[0].removedLines).toEqual(new Set([1]));
    expect(hunks[0].addedLines).toEqual(new Set());
  });

  test("empty original yields a single all-added hunk", () => {
    const hunks = diffFiles("", "a\nb\n");
    expect(hunks).toHaveLength(1);
    expect(hunks[0].pairs).toEqual([
      [null, 0],
      [null, 1],
    ]);
    expect(hunks[0].removedLines).toEqual(new Set());
    expect(hunks[0].addedLines).toEqual(new Set([0, 1]));
  });

  test("emptied content yields a single all-removed hunk", () => {
    const hunks = diffFiles("a\nb\n", "");
    expect(hunks).toHaveLength(1);
    expect(hunks[0].pairs).toEqual([
      [0, null],
      [1, null],
    ]);
    expect(hunks[0].removedLines).toEqual(new Set([0, 1]));
    expect(hunks[0].addedLines).toEqual(new Set());
  });

  test("merges two nearby changes into one contiguous hunk", () => {
    // 10 unchanged lines between the edits is wide enough that jsdiff emits
    // two separate hunks (context only covers 4 lines each side), but the
    // 2-line gap is under the merge threshold, so mergeHunks fills it back
    // into a single contiguous hunk.
    const mid = Array.from({ length: 10 }, (_, i) => `m${i}`).join("\n");
    const tail = Array.from({ length: 4 }, (_, i) => `t${i}`).join("\n");
    const left = `X\n${mid}\nY\n${tail}`;
    const right = `Xchg\n${mid}\nYchg\n${tail}`;

    const hunks = diffFiles(left, right);
    expect(hunks).toHaveLength(1);
    expect(hunks[0].pairs).toEqual(
      Array.from({ length: 16 }, (_, i) => [i, i]),
    );
    expect(hunks[0].removedLines).toEqual(new Set([0, 11]));
    expect(hunks[0].addedLines).toEqual(new Set([0, 11]));
  });

  test("keeps two distant changes as separate hunks", () => {
    // 30 unchanged lines between the edits exceeds the merge threshold,
    // so the two hunks stay separate.
    const mid = Array.from({ length: 30 }, (_, i) => `m${i}`).join("\n");
    const tail = Array.from({ length: 4 }, (_, i) => `t${i}`).join("\n");
    const left = `X\n${mid}\nY\n${tail}`;
    const right = `Xchg\n${mid}\nYchg\n${tail}`;

    const hunks = diffFiles(left, right);
    expect(hunks).toHaveLength(2);
    expect(hunks[0].removedLines).toEqual(new Set([0]));
    expect(hunks[0].addedLines).toEqual(new Set([0]));
    expect(hunks[1].removedLines).toEqual(new Set([31]));
    expect(hunks[1].addedLines).toEqual(new Set([31]));
  });
});
