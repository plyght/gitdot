"use client";

import { ChevronRight, Circle, X } from "lucide-react";
import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";
import type { CommitFilter } from "../util";

const AUTHOR_OPTIONS = ["baepaul", "mikkel"];
const REGEX_PRESETS = ["feat:", "fix:", "chore:", "docs:", "refactor:"];

export function CommitsFilterPanel({
  filters,
  activeFilter,
  setActiveFilter,
  pathOptions,
}: {
  filters: CommitFilter[];
  activeFilter: CommitFilter;
  setActiveFilter: (filter: CommitFilter) => void;
  pathOptions: string[];
}) {
  const active =
    filters.find((f) => f.name === activeFilter.name) ?? filters[0];

  return (
    <div className="flex flex-col w-64 shrink-0 border-l border-border">
      <div className="flex flex-col h-42 shrink-0 border-b border-border">
        <div className="flex items-center h-6 px-2 shrink-0 border-b border-border">
          <span className="text-xs text-muted-foreground font-mono">
            Filters
          </span>
        </div>
        <div className="flex flex-col flex-1 min-h-0 overflow-y-auto">
          {filters.map((filter) => (
            <button
              key={filter.name}
              type="button"
              onClick={() => setActiveFilter(filter)}
              className={cn(
                "w-full flex flex-row items-center h-6 px-2 text-xs text-left transition-colors shrink-0 border-b border-border font-mono",
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
      <FilterDetail
        key={active.name}
        filter={active}
        pathOptions={pathOptions}
      />
    </div>
  );
}

function FilterDetail({
  filter,
  pathOptions,
}: {
  filter: CommitFilter;
  pathOptions: string[];
}) {
  const [authors, setAuthors] = useState<string[]>(filter.authors ?? []);
  const [paths, setPaths] = useState<string[]>(filter.included_paths ?? []);
  const [tags, setTags] = useState<string[]>([]);

  const toggleAuthor = (a: string) =>
    setAuthors((prev) =>
      prev.includes(a) ? prev.filter((x) => x !== a) : [...prev, a],
    );
  const toggleTag = (t: string) =>
    setTags((prev) =>
      prev.includes(t) ? prev.filter((x) => x !== t) : [...prev, t],
    );

  return (
    <div className="flex flex-col flex-1 min-h-0 overflow-y-auto">
      <CriterionRow
        label="Authors"
        count={authors.length}
        summary={authors.length > 0 ? authors.join(", ") : "All authors"}
        content={
          <CheckList
            options={AUTHOR_OPTIONS}
            selected={authors}
            onToggle={toggleAuthor}
          />
        }
      />
      <CriterionRow
        label="Tags"
        count={tags.length}
        summary={tags.length > 0 ? tags.join(", ") : "Any message"}
        content={
          <CheckList
            options={REGEX_PRESETS}
            selected={tags}
            onToggle={toggleTag}
          />
        }
      />
      <PathsRow
        options={pathOptions}
        selected={paths}
        onAdd={(p) =>
          setPaths((prev) => (prev.includes(p) ? prev : [...prev, p]))
        }
        onRemove={(p) => setPaths((prev) => prev.filter((x) => x !== p))}
      />
    </div>
  );
}

function CriterionRow({
  label,
  count,
  summary,
  content,
}: {
  label: string;
  count: number;
  summary: string;
  content: React.ReactNode;
}) {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger className="w-full flex items-start justify-between gap-2 px-2 py-1.5 text-left shrink-0 border-b border-border hover:bg-accent/50 transition-colors focus:outline-none">
        <div className="flex flex-col min-w-0 gap-0.5">
          <span className="text-[10px] text-muted-foreground uppercase tracking-wide font-mono">
            {label}
            {count > 0 ? ` (${count})` : ""}
          </span>
          <span
            className={cn(
              "text-xs font-mono truncate",
              count > 0 ? "text-foreground" : "text-muted-foreground/40",
            )}
          >
            {summary}
          </span>
        </div>
        <ChevronRight className="size-3 text-muted-foreground shrink-0 mt-1" />
      </DropdownMenuTrigger>
      <DropdownMenuContent side="bottom" align="start" className="w-56">
        {content}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

function CheckList({
  options,
  selected,
  onToggle,
}: {
  options: string[];
  selected: string[];
  onToggle: (value: string) => void;
}) {
  if (options.length === 0) {
    return (
      <div className="px-2 py-1.5 text-xs text-muted-foreground font-mono">
        No options
      </div>
    );
  }
  return options.map((opt) => (
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
  ));
}

function PathsRow({
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
  const [focused, setFocused] = useState(false);

  const suggestions = options
    .filter(
      (o) =>
        (query.length === 0 ||
          o.toLowerCase().includes(query.toLowerCase())) &&
        !selected.includes(o),
    )
    .slice(0, 8);

  const showSuggestions = focused && suggestions.length > 0;

  return (
    <div className="relative flex flex-col gap-1 px-2 py-1.5 shrink-0 border-b border-border">
      <span className="text-[10px] text-muted-foreground uppercase tracking-wide font-mono">
        Paths{selected.length > 0 ? ` (${selected.length})` : ""}
      </span>
      {selected.map((path) => (
        <div key={path} className="flex items-center justify-between gap-1">
          <span className="text-xs font-mono text-foreground truncate">
            {path}
          </span>
          <button
            type="button"
            onClick={() => onRemove(path)}
            className="text-muted-foreground hover:text-foreground shrink-0"
          >
            <X className="size-3" />
          </button>
        </div>
      ))}
      <input
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        onFocus={() => setFocused(true)}
        onBlur={() => setTimeout(() => setFocused(false), 150)}
        onKeyDown={(e) => {
          if (e.key === "Enter" && suggestions.length > 0) {
            onAdd(suggestions[0]);
            setQuery("");
          }
        }}
        placeholder="Search paths..."
        className="text-xs bg-transparent placeholder:text-muted-foreground/40 focus:outline-none w-full font-mono text-foreground"
      />
      {showSuggestions && (
        <div className="absolute left-0 right-0 top-full z-10 bg-popover border border-border shadow-md max-h-48 overflow-y-auto">
          {suggestions.map((s) => (
            <button
              key={s}
              type="button"
              onMouseDown={(e) => {
                e.preventDefault();
                onAdd(s);
                setQuery("");
              }}
              className="flex items-center px-2 h-6 w-full text-left font-mono text-xs text-muted-foreground hover:text-foreground hover:bg-accent/50 border-b border-border last:border-b-0"
            >
              {s}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
