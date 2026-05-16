"use client";

import { useEffect, useMemo, useRef } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";

export function CommitShortcuts() {
  const indexRef = useRef(0);

  // when the user scrolls with their mouse, update the index
  useEffect(() => {
    const handler = () => {
      indexRef.current = getIndexFromScroll(getDiffFiles());
    };
    window.addEventListener("wheel", handler, { capture: true, passive: true });
    return () =>
      window.removeEventListener("wheel", handler, { capture: true });
  }, []);

  const shortcuts = useMemo(
    () => [
      {
        name: "NextFile",
        description: "Next file",
        keys: ["j"],
        execute: () => {
          const files = getDiffFiles();
          if (!files.length) return;

          const next = Math.min(indexRef.current + 1, files.length);
          indexRef.current = next;
          navigateTo(next, files);
        },
      },
      {
        name: "PrevFile",
        description: "Previous file",
        keys: ["k"],
        execute: () => {
          const files = getDiffFiles();
          if (!files.length) return;

          const prev = Math.max(indexRef.current - 1, 0);
          indexRef.current = prev;
          navigateTo(prev, files);
        },
      },
    ],
    [],
  );

  useShortcuts(shortcuts);
  return null;
}

function getDiffFiles() {
  return Array.from(document.querySelectorAll<HTMLElement>("[data-diff-file]"));
}

function navigateTo(index: number, files: HTMLElement[]) {
  if (index === 0) {
    document.querySelector<HTMLElement>("[data-diff-top]")?.scrollIntoView();
  } else {
    const target = files[index - 1];
    if (!target) return;
    target.scrollIntoView();
  }
}

function getIndexFromScroll(files: HTMLElement[]) {
  let current = 0;
  for (let i = 0; i < files.length; i++) {
    if (files[i].getBoundingClientRect().top < 0) {
      current = i + 1;
    }
  }
  return current;
}
