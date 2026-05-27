"use client";

import { ClientProvider } from "gitdot-dal/client";

export function DalProvider({ children }: { children: React.ReactNode }) {
  ClientProvider.instance;
  return <>{children}</>;
}
