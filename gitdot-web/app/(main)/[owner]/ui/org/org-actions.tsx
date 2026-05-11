"use client";

import type { OrganizationResource } from "gitdot-api";

export function OrgActions({ org }: { org: OrganizationResource }) {
  const actions: { label: string; onClick: () => void }[] = [
    {
      label: "new repo",
      onClick: () =>
        window.dispatchEvent(
          new CustomEvent("openNewRepo", { detail: { owner: org.name } }),
        ),
    },
    { label: "members", onClick: () => {} },
    { label: "settings", onClick: () => {} },
  ];

  return (
    <div className="flex flex-col items-end">
      <p className="font-semibold text-sm mb-0.5">actions</p>
      {actions.map((action) => (
        <button
          key={action.label}
          type="button"
          onClick={action.onClick}
          className="text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200 cursor-pointer"
        >
          {action.label}
        </button>
      ))}
    </div>
  );
}
