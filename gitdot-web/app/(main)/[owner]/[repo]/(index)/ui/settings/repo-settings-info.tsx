"use client";

import type { RepositoryResource } from "gitdot-api";
import { Save } from "lucide-react";
import { useEffect, useState } from "react";
import { useTimezone } from "@/(main)/context/timezone";
import { toast } from "@/(main)/context/toaster";
import { updateRepositoryAction } from "@/actions";
import { formatDate, timeAgo } from "@/util/date";

export function RepoSettingsInfo({
  repository,
}: {
  repository: RepositoryResource;
}) {
  const tz = useTimezone();
  const [description, setDescription] = useState(repository.description ?? "");
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    setDescription(repository.description ?? "");
  }, [repository]);

  const dirty = description !== (repository.description ?? "");

  async function handleSave() {
    if (!dirty || saving) return;
    setSaving(true);
    const result = await updateRepositoryAction(
      repository.owner,
      repository.name,
      { description },
    );
    setSaving(false);
    if ("error" in result) {
      toast.error(result.error);
      return;
    }
    toast.success("Repository info updated.");
  }

  return (
    <div className="p-4">
      <div className="space-y-6">
        <div className="flex justify-between items-start gap-4">
          <div className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 items-end">
            <span className="text-sm text-muted-foreground">name</span>
            <span className="text-sm">
              <span className="text-muted-foreground">{repository.owner}/</span>
              <span className="font-semibold">{repository.name}</span>
            </span>
            <span className="text-sm text-muted-foreground">created</span>
            <span className="text-sm text-muted-foreground">
              {formatDate(new Date(repository.created_at), tz)} (
              {timeAgo(new Date(repository.created_at))})
            </span>
          </div>
          <button
            type="button"
            onClick={handleSave}
            disabled={!dirty || saving}
            className={`flex items-center gap-1.5 text-sm border-b transition-colors cursor-pointer disabled:cursor-not-allowed ${
              saving ? "border-transparent" : "border-current"
            } ${dirty ? "text-foreground" : "text-muted-foreground"}`}
          >
            <Save className="size-3.5 mb-px" />
            {saving ? "Saving..." : "Save"}
          </button>
        </div>
        <div className="space-y-2">
          <p className="text-xs text-muted-foreground font-mono">
            <span className="text-foreground/40 select-none"># </span>
            description
          </p>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            className="text-sm bg-transparent border-l border-border pl-2 outline-none w-full min-h-24 placeholder:text-muted-foreground/40 transition-colors focus:border-foreground resize-none field-sizing-content"
            placeholder="what this repo is about..."
          />
        </div>
      </div>
    </div>
  );
}
