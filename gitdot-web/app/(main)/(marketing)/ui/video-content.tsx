"use client";

import type { VideoHTMLAttributes } from "react";
import { useState } from "react";
import { cn } from "@/util";

function PlayOverlay() {
  return (
    <div className="absolute inset-0 flex items-center justify-center bg-black/20 group-hover:bg-black/30 transition-colors rounded-lg">
      <div className="bg-white/70 rounded-full p-3.5 group-hover:bg-white/90 transition-colors">
        <svg
          className="w-8 h-8 text-black"
          fill="currentColor"
          viewBox="0 0 24 24"
        >
          <title>Play video</title>
          <path d="M8 5v14l11-7z" />
        </svg>
      </div>
    </div>
  );
}

function VideoDialog({
  src,
  isOpen,
  onClose,
}: {
  src: string;
  isOpen: boolean;
  onClose: () => void;
}) {
  if (!isOpen) return null;

  return (
    <button
      type="button"
      className="fixed inset-0 w-full z-50 flex items-center justify-center bg-black/80 p-4"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
      onKeyDown={(e) => {
        if (e.key === "Escape") onClose();
      }}
      aria-label="Close video dialog"
    >
      <div className="relative w-full max-w-6xl">
        <video src={src} controls autoPlay className="w-full rounded-lg">
          <track kind="captions" />
        </video>
      </div>
    </button>
  );
}

type VideoContentProps = VideoHTMLAttributes<HTMLVideoElement> & {
  node?: unknown;
  /** Play the video inline (autoplay, click to pause) instead of opening a modal. */
  inline?: boolean;
  /** Markdown opt-in: `<video data-inline>` plays inline. */
  "data-inline"?: string | boolean;
};

export function VideoContent({
  src,
  children,
  className,
  inline,
  "data-inline": dataInline,
}: VideoContentProps) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const strSrc = typeof src === "string" ? src : undefined;
  const videoSrc =
    strSrc ||
    (Array.isArray(children)
      ? (children as Array<{ props?: { src?: string } }>).find(
          (child) => child?.props?.src,
        )?.props?.src
      : undefined);

  const playInline = inline || dataInline !== undefined;

  if (playInline) {
    return (
      <video
        src={videoSrc}
        autoPlay
        loop
        muted
        playsInline
        onClick={(e) => {
          const video = e.currentTarget;
          if (video.paused) video.play();
          else video.pause();
        }}
        className={cn("w-full rounded-lg mb-4 cursor-pointer", className)}
      >
        <track kind="captions" />
      </video>
    );
  }

  return (
    <>
      <button
        type="button"
        className={cn(
          "relative cursor-pointer group mb-4 w-full p-0 border-0 bg-transparent",
          className,
        )}
        onClick={() => setIsDialogOpen(true)}
        aria-label="Play video"
      >
        <video src={videoSrc} muted className="w-full rounded-lg">
          <track kind="captions" />
        </video>
        <PlayOverlay />
      </button>
      <VideoDialog
        src={videoSrc ?? ""}
        isOpen={isDialogOpen}
        onClose={() => setIsDialogOpen(false)}
      />
    </>
  );
}
