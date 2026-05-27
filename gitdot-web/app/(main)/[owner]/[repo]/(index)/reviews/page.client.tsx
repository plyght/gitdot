"use client";

import type { ReviewResource } from "gitdot-api";
import { useMemo, useState } from "react";
import { ReviewRow } from "./ui/review-row";

export type ReviewsFilter = "draft" | "open" | "closed" | "all";

function filterReviews(
  reviews: ReviewResource[],
  filter: ReviewsFilter,
): ReviewResource[] {
  const filtered =
    filter === "all" ? reviews : reviews.filter((r) => r.status === filter);

  return filtered.sort(
    (a, b) =>
      new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
  );
}

export function PageClient({
  owner,
  repo,
  reviews,
}: {
  owner: string;
  repo: string;
  reviews: ReviewResource[] | null;
}) {
  const [filter, setFilter] = useState<ReviewsFilter>("all");

  const filteredReviews = useMemo(
    () => filterReviews(reviews ?? [], filter),
    [reviews, filter],
  );

  if (!reviews) return null;

  return (
    <div className="flex flex-col">
      <div className="flex flex-row items-center gap-2 px-4 h-10 border-b text-sm">
        {(["all", "draft", "open", "closed"] as const).map((f) => (
          <button
            key={f}
            type="button"
            onClick={() => setFilter(f)}
            className={`cursor-default ${filter === f ? "text-foreground" : "text-muted-foreground hover:text-foreground"}`}
          >
            {f}
          </button>
        ))}
      </div>
      {filteredReviews.map((review) => (
        <ReviewRow key={review.id} owner={owner} repo={repo} review={review} />
      ))}
    </div>
  );
}
