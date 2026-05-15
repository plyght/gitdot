"use client";

import type { CommitFilterResource } from "gitdot-api";
import { ChevronDown, Circle, X } from "lucide-react";
import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";

const AUTHOR_OPTIONS = ["baepaul", "mikkel"];
const REGEX_PRESETS = ["feat:", "fix:", "chore:", "docs:", "refactor:"];

export function CommitsFilterPanel({
  filters,
  activeFilter,
  setActiveFilter,
  pathOptions,
}: {
  filters: CommitFilterResource[];
  activeFilter: CommitFilterResource;
  setActiveFilter: (filter: CommitFilterResource) => void;
  pathOptions: string[];
}) {
  const active =
    filters.find((f) => f.name === activeFilter.name) ?? filters[0];

  return (
    <div className="flex flex-col w-64 shrink-0 border-l border-border">
      <div className="flex flex-col min-h-42 shrink-0 border-b border-border">
      <div className="flex items-center h-6 px-2 shrink-0 border-b border-border">
        <span className="text-xs text-muted-foreground font-mono">Filters</span>
      </div>
      <div className="flex flex-col flex-1 min-h-0 overflow-y-auto">
        {filters.map((filter) => (
          <button
            key={filter.name}
            type="button"
            onClick={() => setActiveFilter(filter)}
            className={cn(
              "w-full flex flex-row items-center h-6 px-2 text-xs text-left transition-colors shrink-0 border-b border-border",
              activeFilter.name === filter.name
                ? "bg-accent text-foreground"
                : "text-muted-foreground hover:bg-accent/50 hover:text-foreground",
            )}
          >
            {filter.name}
          </button>
        ))}
      </div>
      </div>
      <FilterDetail key={active.name} filter={active} pathOptions={pathOptions} />
    </div>
  );
}

function FilterDetail({
  filter,
  pathOptions,
}: {
  filter: CommitFilterResource;
  pathOptions: string[];
}) {
  const [authors, setAuthors] = useState<string[]>(filter.authors ?? []);
  const [paths, setPaths] = useState<string[]>(filter.included_paths ?? []);
  const [tags, setTags] = useState<string[]>([]);

  const toggleAuthor = (a: string) =>
    setAuthors((prev) =>
      prev.includes(a) ? prev.filter((x) => x !== a) : [...prev, a],
    );

return (
    <div className="flex flex-col flex-1 min-h-0 px-2 py-2 gap-2 overflow-y-auto">
      <div className="flex flex-col gap-1.5">
        <MultiSelectField
          label="Authors"
          options={AUTHOR_OPTIONS}
          selected={authors}
          onToggle={toggleAuthor}
          placeholder="All authors"
        />
        <MultiSelectField
          label="Tags"
          options={REGEX_PRESETS}
          selected={tags}
          onToggle={(t) =>
            setTags((prev) =>
              prev.includes(t) ? prev.filter((x) => x !== t) : [...prev, t],
            )
          }
          placeholder="Any message"
        />
        <PathListField
          options={pathOptions}
          selected={paths}
          onAdd={(p) =>
            setPaths((prev) => (prev.includes(p) ? prev : [...prev, p]))
          }
          onRemove={(p) => setPaths((prev) => prev.filter((x) => x !== p))}
        />
      </div>
    </div>
  );
}

function MultiSelectField({
  label,
  options,
  selected,
  onToggle,
  placeholder,
}: {
  label: string;
  options: string[];
  selected: string[];
  onToggle: (value: string) => void;
  placeholder: string;
}) {
  const displayValue = selected.length > 0 ? selected.join(", ") : null;

  return (
    <div className="flex flex-col gap-0.5">
      <span className="text-[10px] text-muted-foreground uppercase tracking-wide font-mono">
        {label}
      </span>
      <DropdownMenu>
        <DropdownMenuTrigger className="flex items-center justify-between w-full text-xs border border-border px-1.5 py-0.5 text-left focus:outline-none hover:border-foreground/50 transition-colors">
          <span
            className={cn(
              "truncate",
              displayValue ? "text-foreground" : "text-muted-foreground/40",
            )}
          >
            {displayValue ?? placeholder}
          </span>
          <ChevronDown className="size-3 text-muted-foreground shrink-0 ml-1" />
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="w-56">
          {options.length === 0 ? (
            <div className="px-2 py-1.5 text-xs text-muted-foreground font-mono">
              No options
            </div>
          ) : (
            options.map((opt) => (
              <DropdownMenuItem
                key={opt}
                onSelect={(e) => e.preventDefault()}
                onClick={() => onToggle(opt)}
                className="text-xs font-mono"
              >
                <Circle
                  className={cn(
                    "size-1.5 shrink-0",
                    selected.includes(opt)
                      ? "fill-current text-foreground"
                      : "fill-none text-muted-foreground",
                  )}
                />
                {opt}
              </DropdownMenuItem>
            ))
          )}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}

function PathListField({
  options,
  selected,
  onAdd,
  onRemove,
}: {
  options: string[];
  selected: string[];
  onAdd: (path: string) => void;
  onRemove: (path: string) => void;
}) {
  const [query, setQuery] = useState("");

  const suggestions = query.length > 0
    ? options
        .filter(
          (o) =>
            o.toLowerCase().includes(query.toLowerCase()) &&
            !selected.includes(o),
        )
        .slice(0, 8)
    : [];

  return (
    <div className="flex flex-col gap-0.5">
      <span className="text-[10px] text-muted-foreground uppercase tracking-wide font-mono">
        Paths
      </span>
      <div className="flex flex-col border border-border text-xs">
        {selected.map((path) => (
          <div
            key={path}
            className="flex items-center justify-between px-1.5 h-6 border-b border-border"
          >
            <span className="font-mono text-foreground truncate">{path}</span>
            <button
              type="button"
              onClick={() => onRemove(path)}
              className="text-muted-foreground hover:text-foreground shrink-0 ml-1"
            >
              <X className="size-3" />
            </button>
          </div>
        ))}
        <input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter" && suggestions.length > 0) {
              onAdd(suggestions[0]);
              setQuery("");
            }
          }}
          placeholder="Add path..."
          className="px-1.5 py-0.5 bg-transparent placeholder:text-muted-foreground/40 focus:outline-none w-full font-mono"
        />
        {suggestions.map((s) => (
          <button
            key={s}
            type="button"
            onMouseDown={(e) => {
              e.preventDefault();
              onAdd(s);
              setQuery("");
            }}
            className="flex items-center px-1.5 h-6 text-left font-mono text-muted-foreground hover:text-foreground hover:bg-accent/50 border-t border-border"
          >
            {s}
          </button>
        ))}
      </div>
    </div>
  );
}
