"use client";

import { useSelectedLayoutSegments } from "next/navigation";
import { useUserContext } from "@/(main)/context/user";
import { useMetricsContext } from "@/context/metrics";
import { useAnimateNumber } from "@/hooks/use-animate-number";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { MainCommandBar } from "./main-command-bar";

export function MainHeader() {
  const segments = useSelectedLayoutSegments();
  if (segments[2] === "reviews" && segments[3] !== undefined) return null;

  return (
    <div className="relative shrink-0 flex w-full h-7 items-center border-b bg-sidebar text-xs font-mono">
      <MainCommandBar />
      <div className="ml-auto flex items-baseline pr-2">
        <AuthStatus />
      </div>
    </div>
  );
}

function AuthStatus() {
  const { user } = useUserContext();

  if (user === undefined) return null;

  if (user) {
    return (
      <Link
        href={`/${user.name}`}
        className="text-muted-foreground hover:text-foreground hover:underline transition-colors duration-200 mr-1.5"
      >
        logged in as {user.name}
      </Link>
    );
  }

  return (
    <button
      type="button"
      onClick={() => window.dispatchEvent(new Event("toggleAuthDialog"))}
      className="text-muted-foreground hover:text-foreground hover:underline transition-colors duration-200 mr-1.5 cursor-pointer"
    >
      browsing as guest <span className="text-foreground">(login)</span>
    </button>
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
