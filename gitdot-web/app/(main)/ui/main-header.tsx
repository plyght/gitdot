"use client";

import type { UserResource } from "gitdot-api";
import { useSelectedLayoutSegments } from "next/navigation";
import { useEffect, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { useMetricsContext } from "@/context/metrics";
import { useAnimateNumber } from "@/hooks/use-animate-number";
import { useTypewriter } from "@/hooks/use-typewriter";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { cn } from "@/util";
import { MainCommandBar } from "./main-command-bar";

export function MainHeader() {
  const segments = useSelectedLayoutSegments();
  if (segments[2] === "reviews" && segments[3] !== undefined) return null;

  return (
    <div className="relative shrink-0 flex w-full h-7 items-center border-b bg-sidebar text-xs font-mono">
      <MainCommandBar />
      <div className="ml-auto flex items-center pr-2">
        <AuthStatus />
      </div>
    </div>
  );
}

function AuthStatus() {
  const { user } = useUserContext();

  if (user === undefined) return null;
  if (user) return <AuthStatusLoggedIn user={user} />;
  return <AuthStatusGhost />;
}

function useTypewriterDone(text: string, speed = 25) {
  const typed = useTypewriter(text, speed);
  const [done, setDone] = useState(false);

  useEffect(() => {
    if (typed !== text) {
      setDone(false);
      return;
    }
    const t = setTimeout(() => setDone(true), 60);
    return () => clearTimeout(t);
  }, [typed, text]);

  return { typed, done };
}

function AuthStatusLoggedIn({ user }: { user: UserResource }) {
  const text = `logged in as ${user.name}`;
  const { typed, done } = useTypewriterDone(text);

  return (
    <Link
      href={`/${user.name}`}
      className="flex items-center gap-2 text-muted-foreground hover:text-foreground hover:underline transition-colors duration-200 mr-1.5"
    >
      <span
        className="inline-block whitespace-pre text-left"
        style={{ width: `${text.length}ch` }}
      >
        {typed}
      </span>
      <span
        className={cn(
          "transition-opacity duration-300",
          done ? "opacity-100" : "opacity-0",
        )}
      >
        <UserImage userId={user.id} px={18} />
      </span>
    </Link>
  );
}

function AuthStatusGhost() {
  const text = "browsing as guest";
  const { typed, done } = useTypewriterDone(text);

  return (
    <div className="flex items-center gap-2 text-muted-foreground mr-1.5">
      <span
        className="inline-block whitespace-pre text-left"
        style={{ width: `${text.length}ch` }}
      >
        {typed}
      </span>
      <span
        className={cn(
          "text-foreground hover:underline transition-opacity duration-300 cursor-pointer",
          done ? "opacity-100" : "opacity-0",
        )}
        onClick={() => window.dispatchEvent(new Event("toggleAuthDialog"))}
      >
        (login)
      </span>
    </div>
  );
}

function PageVitals() {
  const { FCP, TTFB, CLS, INP } = useMetricsContext();
  const animatedFCP = useAnimateNumber(FCP);

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="w-[5ch] text-center text-muted-foreground hover:text-foreground transition-colors outline-none cursor-pointer select-none p-0 leading-none"
        >
          {animatedFCP != null ? `${animatedFCP}ms` : "0ms"}
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        side="bottom"
        align="end"
        sideOffset={12}
        alignOffset={-8}
      >
        <div className="px-2 py-1.5 text-xs font-mono space-y-1">
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">FCP</span>
            <span>{FCP != null ? `${Math.round(FCP)}ms` : "-"}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">TTFB</span>
            <span>{TTFB != null ? `${Math.round(TTFB)}ms` : "-"}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">CLS</span>
            <span>{CLS != null ? CLS.toFixed(3) : "-"}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">INP</span>
            <span>{INP != null ? `${Math.round(INP)}ms` : "-"}</span>
          </div>
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
