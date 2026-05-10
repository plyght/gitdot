"use client";

import { Building2 } from "lucide-react";
import Image from "next/image";
import { useState } from "react";

export function OrgImage({ orgId, px = 32 }: { orgId?: string; px?: number }) {
  const [errored, setErrored] = useState(false);

  if (!orgId || errored) {
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
      onError={() => setErrored(true)}
    />
  );
}
