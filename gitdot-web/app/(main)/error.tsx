"use client";

import Link from "@/ui/link";

export default function ErrorPage({
  error,
  unstable_retry,
}: {
  error: Error & { digest?: string };
  unstable_retry: () => void;
}) {
  return (
    <div className="flex flex-col items-center justify-center h-full w-full gap-2 p-4">
      <p className="text-sm font-mono text-foreground">{error.message}</p>
      <div className="flex flex-col gap-1 items-center">
        <button
          type="button"
          className="text-xs text-muted-foreground hover:text-foreground transition-colors duration-200 cursor-pointer underline lowercase"
          onClick={() => unstable_retry()}
        >
          try again
        </button>
        <Link
          href={"/"}
          className="text-xs text-muted-foreground hover:text-foreground transition-colors duration-200 underline lowercase"
        >
          return home
        </Link>
      </div>
    </div>
  );
}
