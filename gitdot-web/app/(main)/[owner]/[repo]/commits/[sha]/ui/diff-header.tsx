import { ChevronDown, ChevronRight } from "lucide-react";

export function DiffHeader({
  open,
  setOpen,
  path,
  linesAdded,
  linesRemoved,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  path: string;
  linesAdded: number;
  linesRemoved: number;
}) {
  return (
    <div
      data-diff-toggle
      id={path}
      onClick={() => setOpen(!open)}
      className="group flex flex-row w-full h-7 shrink-0 items-center px-2 text-xs font-mono bg-sidebar hover:bg-sidebar-accent/80 border-b border-border select-none cursor-pointer transition-colors duration-200"
    >
      <div className="flex flex-row items-center gap-2 mr-auto">
        <span data-diff-path className="text-muted-foreground">
          {path}
        </span>
        <span className="flex flex-row font-mono select-none gap-1">
          <span className="text-green-600">+{linesAdded}</span>
          <span className="text-red-600">-{linesRemoved}</span>
        </span>
      </div>
      {open ? (
        <ChevronDown className="size-3 text-muted-foreground group-hover:text-foreground transition-colors duration-200 shrink-0" />
      ) : (
        <ChevronRight className="size-3 text-muted-foreground group-hover:text-foreground transition-colors duration-200 shrink-0" />
      )}
    </div>
  );
}
