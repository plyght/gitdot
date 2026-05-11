"use client";

import { Building2 } from "lucide-react";
import Image from "next/image";

export function OrgImage({ orgId, px = 32 }: { orgId?: string; px?: number }) {
  if (!orgId) {
    return (
      <Building2
        className="shrink-0 text-muted-foreground"
        style={{ width: px, height: px }}
      />
    );
  }

  return (
    <Image
      src={`https://images.gitdot.io/orgs/${orgId}.webp`}
      alt="organization avatar"
      width={px}
      height={px}
      className="rounded-full shrink-0"
      style={{ width: px, height: px }}
      unoptimized
    />
  );
}
