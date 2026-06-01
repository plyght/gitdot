"use client";

import { ChevronDown } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";

export function UserCommitsHeader({
  view,
  years,
  onSelect,
}: {
  view: "recent" | number;
  years: number[];
  onSelect: (view: "recent" | number) => void;
}) {
  const label = view === "recent" ? "Recent" : String(view);

  return (
    <div className="flex items-baseline mb-2 justify-between px-3">
      <span className="text-xs text-muted-foreground font-mono">
        <span className="text-foreground/40 select-none"># </span>Commits
      </span>
      <DropdownMenu>
        <DropdownMenuTrigger className="flex items-center gap-0.5 text-xs text-muted-foreground/60 font-mono cursor-pointer transition-colors hover:text-foreground">
          {label}
          <ChevronDown className="size-3" />
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="min-w-20">
          <DropdownMenuItem
            className="text-xs font-mono"
            onClick={() => onSelect("recent")}
          >
            Recent
          </DropdownMenuItem>
          {years.map((y) => (
            <DropdownMenuItem
              key={y}
              className="text-xs font-mono"
              onClick={() => onSelect(y)}
            >
              {y}
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
