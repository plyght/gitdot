"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import { MarkdownBody } from "../ui/markdown/markdown-body";
import type { Resources } from "./page";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <div className="flex-1 min-w-0 overflow-y-auto scrollbar-none">
      <Suspense fallback={<Loading />}>
        <Readme promise={resolvedPromises.readme} />
      </Suspense>
    </div>
  );
}

function Readme({ promise }: { promise: ResourcePromises["readme"] }) {
  const readme = use(promise);
  return (
    <div className="p-4 w-full">
      {readme && readme.type === "file" ? (
        <MarkdownBody content={readme.content} />
      ) : (
        <div className="p-2 text-sm">README.md not found</div>
      )}
    </div>
  );
}
