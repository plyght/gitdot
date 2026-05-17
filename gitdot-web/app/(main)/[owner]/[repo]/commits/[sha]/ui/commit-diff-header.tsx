export function CommitDiffHeader({
  path,
  linesAdded,
  linesRemoved,
}: {
  path: string;
  linesAdded: number;
  linesRemoved: number;
}) {
  return (
    <div
      id={path}
      className="flex flex-row w-full h-7 shrink-0 items-center px-2 text-xs font-mono bg-sidebar border-b border-border select-none scroll-mt-4"
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
    </div>
  );
}
