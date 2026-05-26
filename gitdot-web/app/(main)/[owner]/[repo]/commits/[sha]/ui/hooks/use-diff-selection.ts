"use client";

import { useCallback, useRef } from "react";

export function useDiffSelection() {
  const containerRef = useRef<HTMLDivElement>(null);
  const allSpansRef = useRef<HTMLElement[]>([]);
  const dragStartRef = useRef<HTMLElement | null>(null);
  const dragEndRef = useRef<HTMLElement | null>(null);

  const clearSelection = useCallback(() => {
    const container = containerRef.current;
    if (!container) return;
    container.classList.remove("has-selection");
    for (const el of container.querySelectorAll<HTMLElement>(
      ".diff-token.token-selected",
    )) {
      el.classList.remove("token-selected");
    }
  }, []);

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      const container = containerRef.current;
      if (!container) return;

      clearSelection();
      dragStartRef.current = null;

      const token =
        e.target instanceof HTMLElement
          ? (e.target.closest<HTMLElement>(".diff-token") ?? null)
          : null;
      if (!token) return;

      container.classList.add("is-dragging");
      allSpansRef.current = Array.from(
        container.querySelectorAll(".diff-token"),
      ) as HTMLElement[];
      e.preventDefault();
      dragStartRef.current = token;
      token.classList.add("token-selected");
      container.classList.add("has-selection");
    },
    [clearSelection],
  );

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (!(e.buttons & 1)) return;
    if (!dragStartRef.current) return;

    const token = getTokenSpan(e.target);
    if (!token) return;

    const spans = allSpansRef.current;
    const startIdx = spans.indexOf(dragStartRef.current);
    const endIdx = spans.indexOf(token);
    if (startIdx === -1 || endIdx === -1) return;

    dragEndRef.current = token;

    const [from, to] =
      startIdx <= endIdx ? [startIdx, endIdx] : [endIdx, startIdx];
    for (let i = 0; i < spans.length; i++) {
      const isInRange = i >= from && i <= to;
      spans[i].classList.toggle("token-selected", isInRange);
    }
  }, []);

  const handleMouseUp = useCallback(() => {
    containerRef.current?.classList.remove("is-dragging");
    dragStartRef.current = null;
    dragEndRef.current = null;
  }, []);

  return {
    containerRef,
    handleMouseDown,
    handleMouseMove,
    handleMouseUp,
  };
}

const getTokenSpan = (target: EventTarget | null): HTMLElement | null => {
  if (!(target instanceof HTMLElement)) return null;
  const direct = target.closest(".diff-token");
  if (direct) return direct as HTMLElement;
  const line = target.closest(".diff-line");
  if (!line) return null;
  const tokens = line.querySelectorAll<HTMLElement>(".diff-token");
  return tokens.length ? tokens[tokens.length - 1] : null;
};
