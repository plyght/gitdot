"use client";

import { Circle } from "lucide-react";
import { useEffect, useState } from "react";
import { useTypewriter } from "@/hooks/use-typewriter";

const DOT_FADE_MS = 400;
const POST_TITLE_PAUSE_MS = 500;
const PARAGRAPH_EXPAND_MS = 300;
const POST_EXPAND_PAUSE_MS = 300;

const NARROW_WIDTH = "6rem";
const WIDE_WIDTH = "36rem";
const PARAGRAPH = `gitdot is a home for open-source maintainers — a faster repo browser, sharper issue triage, and a notifications inbox that respects your attention. Pick a username, push your first repo, and we'll get out of your way. Everything you can do in the browser, you can do from the CLI; pushes and clones run over standard git, and we never throttle them.`;

export default function Page() {
  const [dotVisible, setDotVisible] = useState(false);
  const [titleVisible, setTitleVisible] = useState(false);
  const [paragraphExpanded, setParagraphExpanded] = useState(false);
  const [paragraphVisible, setParagraphVisible] = useState(false);
  const [nextVisible, setNextVisible] = useState(false);

  const title = useTypewriter(titleVisible ? "Welcome." : "", 25);
  const titleDone = titleVisible && title === "Welcome.";
  const paragraph = useTypewriter(paragraphVisible ? PARAGRAPH : "", 8);
  const paragraphDone = paragraphVisible && paragraph === PARAGRAPH;

  useEffect(() => {
    const fadeIn = setTimeout(() => setDotVisible(true), 50);
    const begin = setTimeout(() => setTitleVisible(true), DOT_FADE_MS + 100);
    return () => {
      clearTimeout(fadeIn);
      clearTimeout(begin);
    };
  }, []);

  useEffect(() => {
    if (!titleDone) return;
    const expand = setTimeout(
      () => setParagraphExpanded(true),
      POST_TITLE_PAUSE_MS,
    );
    const startPara = setTimeout(
      () => setParagraphVisible(true),
      POST_TITLE_PAUSE_MS + PARAGRAPH_EXPAND_MS + POST_EXPAND_PAUSE_MS,
    );
    return () => {
      clearTimeout(expand);
      clearTimeout(startPara);
    };
  }, [titleDone]);

  useEffect(() => {
    if (!paragraphDone) return;
    const t = setTimeout(() => setNextVisible(true), 200);
    return () => clearTimeout(t);
  }, [paragraphDone]);

  return (
    <div className="min-h-screen flex items-center justify-center pb-[10vh]">
      <div
        className="text-sm space-y-1 transition-[width] ease-out"
        style={{
          width: paragraphExpanded ? WIDE_WIDTH : NARROW_WIDTH,
          transitionDuration: `${PARAGRAPH_EXPAND_MS}ms`,
        }}
      >
        <p className="font-mono flex items-center gap-2 min-h-[1.5em]">
          <Circle
            size={8}
            className="fill-current transition-opacity duration-400"
            style={{ opacity: dotVisible ? 1 : 0 }}
          />
          <span>{title}</span>
        </p>
        <p className="min-h-[5.5em] whitespace-pre-wrap">{paragraph}</p>
        <div
          className="flex justify-end transition-opacity duration-500"
          style={{ opacity: nextVisible ? 1 : 0 }}
        >
          <button
            type="button"
            disabled={!nextVisible}
            className="cursor-pointer underline decoration-current text-muted-foreground hover:text-foreground transition-colors duration-200 disabled:cursor-not-allowed"
          >
            Next.
          </button>
        </div>
      </div>
    </div>
  );
}
