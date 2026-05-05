"use client";

import type {
  RepositoryPathResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { useParams } from "next/navigation";
import {
  Fragment,
  Suspense,
  use,
  useCallback,
  useLayoutEffect,
  useRef,
  useState,
} from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import Link from "@/ui/link";
import { Loading } from "@/ui/loading";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { cn } from "@/util";
import type { Resources } from "./layout";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function LayoutClient({
  owner,
  repo,
  requests,
  promises,
  children,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
  children: React.ReactNode;
}) {
  const { path } = useParams<{ path: string[] }>();
  const filePath = path.join("/") ?? "";
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);

  return (
    <>
      <Sidebar>
        <SidebarContent className="overflow-auto flex flex-col w-full">
          <div className="flex flex-col w-full">
            <Suspense fallback={<Loading />}>
              <FileTree
                owner={owner}
                repo={repo}
                filePath={filePath}
                promises={resolvedPromises}
              />
            </Suspense>
          </div>
        </SidebarContent>
      </Sidebar>
      <Suspense>
        <OverlayScroll>{children}</OverlayScroll>
      </Suspense>
    </>
  );
}

function FileTree({
  owner,
  repo,
  filePath,
  promises,
}: {
  owner: string;
  repo: string;
  filePath: string;
  promises: ResourcePromises;
}) {
  const paths = use(promises.paths);

  if (!paths) return null;

  return (
    <FileTreeRows owner={owner} repo={repo} filePath={filePath} paths={paths} />
  );
}

function FileTreeRows({
  owner,
  repo,
  filePath,
  paths,
}: {
  owner: string;
  repo: string;
  filePath: string;
  paths: RepositoryPathsResource;
}) {
  const [expandedFolders, setExpandedFolders] = useState<Set<string>>(
    new Set(),
  );
  const expandedFoldersRef = useRef(expandedFolders);
  expandedFoldersRef.current = expandedFolders;

  const toggleFolder = (path: string) => {
    setExpandedFolders((prev) => {
      const next = new Set(prev);
      if (next.has(path)) {
        for (const p of next) {
          if (p === path || p.startsWith(`${path}/`)) {
            next.delete(p);
          }
        }
      } else {
        next.add(path);
      }
      return next;
    });
  };

  const expandFolders = useCallback((paths: string[]) => {
    setExpandedFolders((prev) => new Set([...prev, ...paths]));
  }, []);

  useLayoutEffect(() => {
    if (filePath === "") return;

    const isFolder = paths.entries.some(
      (e) => e.path === filePath && e.path_type === "tree",
    );
    const segments = filePath.split("/");
    const targetSegments = isFolder ? segments : segments.slice(0, -1);
    const ancestors = targetSegments.map((_, i) =>
      targetSegments.slice(0, i + 1).join("/"),
    );
    if (
      ancestors.length > 0 &&
      !ancestors.every((a) => expandedFoldersRef.current.has(a))
    ) {
      expandFolders(ancestors);
    }
  }, [filePath, paths, expandFolders]);

  const renderRows = (parentPath: string, depth: number): React.ReactNode => {
    const entries = getFolderEntries(parentPath, paths);
    return entries.map((entry) => {
      const isFolder = entry.path_type === "tree";
      const isExpanded = expandedFolders.has(entry.path);
      const isActive = filePath === entry.path;

      return (
        <Fragment key={entry.path}>
          {isFolder ? (
            <FolderRow
              owner={owner}
              repo={repo}
              entry={entry}
              depth={depth}
              isActive={isActive}
              expanded={isExpanded}
              setExpanded={() => toggleFolder(entry.path)}
            />
          ) : (
            <FileRow
              owner={owner}
              repo={repo}
              entry={entry}
              depth={depth}
              isActive={isActive}
            />
          )}
          {isFolder && isExpanded && renderRows(entry.path, depth + 1)}
        </Fragment>
      );
    });
  };

  return <>{renderRows("", 0)}</>;
}

function RowGutter({ depth }: { depth: number }) {
  if (depth === 0) return null;
  return (
    <>
      {Array.from({ length: depth }, (_, i) => {
        const left = 10 + i * 16;
        return (
          <span
            key={`gutter-${left}px`}
            className="absolute top-0 bottom-0 w-px bg-border"
            style={{ left: `${left}px` }}
          />
        );
      })}
    </>
  );
}

function FolderRow({
  owner,
  repo,
  depth,
  entry,
  isActive,
  expanded,
  setExpanded,
}: {
  owner: string;
  repo: string;
  depth: number;
  entry: RepositoryPathResource;
  isActive: boolean;
  expanded: boolean;
  setExpanded: () => void;
}) {
  const name = entry.path.split("/").pop();

  return (
    <button
      type="button"
      onClick={() => setExpanded()}
      style={{ paddingLeft: `${8 + depth * 16}px` }}
      className={cn(
        "relative flex flex-row w-full h-8 items-center select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2",
        isActive &&
          "bg-sidebar border-b border-b-border border-t border-t-border",
      )}
      data-sidebar-item=""
      data-sidebar-item-active={isActive ? "true" : undefined}
    >
      <RowGutter depth={depth} />
      <Link
        href={`/${owner}/${repo}/${entry.path}`}
        onClick={(e) => e.stopPropagation()}
        className="inline-flex items-center truncate cursor-pointer"
      >
        <span className="underline decoration-transparent hover:decoration-current">
          {name}
        </span>
        {expanded && "/"}
      </Link>
    </button>
  );
}

function FileRow({
  owner,
  repo,
  depth,
  entry,
  isActive,
}: {
  owner: string;
  repo: string;
  depth: number;
  entry: RepositoryPathResource;
  isActive: boolean;
}) {
  const name = entry.path.split("/").pop();

  return (
    <Link
      href={`/${owner}/${repo}/${entry.path}`}
      style={{ paddingLeft: `${8 + depth * 16}px` }}
      className={cn(
        "relative flex flex-row w-full h-8 items-center select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2",
        isActive &&
          "bg-sidebar border-b border-b-border border-t border-t-border",
      )}
      data-sidebar-item=""
      data-sidebar-item-active={isActive}
    >
      <RowGutter depth={depth} />
      <span className="truncate">{name}</span>
    </Link>
  );
}
