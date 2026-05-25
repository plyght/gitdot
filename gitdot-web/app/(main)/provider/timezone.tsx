"use client";

import { createContext, useContext, useEffect, useState } from "react";

const TimezoneContext = createContext<string>("UTC");

// TODO: set and receive cookies if ip address flicker is common
export function TimezoneProvider({
  initialTimezone,
  children,
}: {
  initialTimezone: string;
  children: React.ReactNode;
}) {
  const [tz, setTz] = useState(initialTimezone);

  useEffect(() => {
    const real = Intl.DateTimeFormat().resolvedOptions().timeZone;
    if (real && real !== initialTimezone) {
      setTz(real);
    }
  }, [initialTimezone]);

  return (
    <TimezoneContext.Provider value={tz}>{children}</TimezoneContext.Provider>
  );
}

export function useTimezone(): string {
  return useContext(TimezoneContext);
}
