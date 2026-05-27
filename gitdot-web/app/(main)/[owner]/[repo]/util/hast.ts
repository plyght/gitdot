import type { RepositoryFileResource } from "gitdot-api";
import { fileToHast, getHighlighter, inferLanguage } from "gitdot-dal/client";
import type { Element, ElementContent, Root } from "hast";
import type { BundledLanguage } from "shiki";

const VITESSE_THEMES = {
  light: "vitesse-light",
  dark: "vitesse-dark",
} as const;

export async function renderFileToHtml(
  file: RepositoryFileResource,
  theme: "vitesse" | "gitdot",
): Promise<string> {
  const lang = inferLanguage(file.path);
  const highlighter = await getHighlighter(lang, theme);

  if (theme === "vitesse") {
    return highlighter.codeToHtml(file.content, {
      lang: lang ?? "plaintext",
      themes: VITESSE_THEMES,
      defaultColor: "light",
    });
  }

  return highlighter.codeToHtml(file.content, {
    lang: lang ?? "plaintext",
    theme: "gitdot-light",
  });
}

export async function renderSpans(
  side: "left" | "right",
  content: string,
  lang: BundledLanguage | undefined,
  changedLines: Set<number>,
  theme: "vitesse" | "gitdot" = "vitesse",
): Promise<Element[]> {
  const hast = await fileToHast(content, lang, theme, [
    {
      pre(node) {
        this.addClassToHast(node, "outline-none");
      },
      code(node) {
        this.addClassToHast(node, "flex flex-col");
      },
      line(node, lineNumber) {
        node.type = "element";
        node.tagName = "diffline";
        node.properties["data-line-number"] = lineNumber;

        if (changedLines.has(lineNumber - 1)) {
          node.properties["data-line-type"] =
            side === "left" ? "removed" : "added";
        } else {
          node.properties["data-line-type"] = "normal";
        }
      },
    },
  ]);

  const root = hast as Root;
  const pre = root.children[0] as Element;
  const code = pre.children[0] as Element;

  const lines = code.children.filter(
    (child): child is Element => child.type === "element",
  );
  for (const line of lines) {
    splitLineByWhitespace(line);
  }
  return lines;
}

function getTextLength(node: ElementContent): number {
  if (node.type === "text") return node.value.length;
  if (node.type === "element") {
    return node.children.reduce((sum, c) => sum + getTextLength(c), 0);
  }
  return 0;
}

function splitLineByWhitespace(lineNode: Element): void {
  const newChildren: ElementContent[] = [];
  let charOffset = 0;

  for (const child of lineNode.children) {
    if (child.type !== "element") {
      newChildren.push(child);
      continue;
    }

    const spanChildren = [...child.children];

    let leadingSpaces = "";
    const firstChild = spanChildren[0];
    if (firstChild?.type === "text") {
      const [spaces, rest] = takeLeadingSpaces(firstChild.value);
      if (spaces) {
        leadingSpaces = spaces;
        if (rest) spanChildren[0] = { type: "text", value: rest };
        else spanChildren.shift();
      }
    }

    let trailingSpaces = "";
    const lastChild = spanChildren[spanChildren.length - 1];
    if (lastChild?.type === "text") {
      const [rest, spaces] = takeTrailingSpaces(lastChild.value);
      if (spaces) {
        trailingSpaces = spaces;
        if (rest)
          spanChildren[spanChildren.length - 1] = { type: "text", value: rest };
        else spanChildren.pop();
      }
    }

    const contentLength = spanChildren.reduce(
      (sum, c) => sum + getTextLength(c),
      0,
    );

    const makeToken = (
      props: Record<string, unknown>,
      c: ElementContent[],
    ): Element => ({
      ...child,
      properties: {
        ...child.properties,
        ...(child.properties.class
          ? { class: [...(child.properties.class as string[]), "diff-token"] }
          : { class: ["diff-token"] }),
        ...props,
      },
      children: c,
    });

    if (leadingSpaces) {
      newChildren.push(
        makeToken(
          {
            "data-char-start": charOffset,
            "data-char-end": charOffset + leadingSpaces.length,
          },
          [{ type: "text", value: leadingSpaces }],
        ),
      );
      charOffset += leadingSpaces.length;
    }
    if (spanChildren.length > 0) {
      newChildren.push(
        makeToken(
          {
            "data-char-start": charOffset,
            "data-char-end": charOffset + contentLength,
          },
          spanChildren,
        ),
      );
      charOffset += contentLength;
    }
    if (trailingSpaces) {
      newChildren.push(
        makeToken(
          {
            "data-char-start": charOffset,
            "data-char-end": charOffset + trailingSpaces.length,
          },
          [{ type: "text", value: trailingSpaces }],
        ),
      );
      charOffset += trailingSpaces.length;
    }
  }

  lineNode.children = newChildren;
}

function takeLeadingSpaces(value: string): [spaces: string, rest: string] {
  let i = 0;
  while (i < value.length && (value[i] === " " || value[i] === "\t")) i++;
  return [value.slice(0, i), value.slice(i)];
}

function takeTrailingSpaces(value: string): [rest: string, spaces: string] {
  let i = value.length;
  while (i > 0 && (value[i - 1] === " " || value[i - 1] === "\t")) i--;
  return [value.slice(0, i), value.slice(i)];
}
