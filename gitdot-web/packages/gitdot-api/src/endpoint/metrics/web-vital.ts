import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const WebVitalEvent = z.object({
  event_time: z.number(),
  name: z.string(),
  value: z.number(),
  rating: z.string(),
  metric_id: z.string(),
  navigation_type: z.string().optional(),
  route: z.string(),
  path: z.string(),
});
export type WebVitalEvent = z.infer<typeof WebVitalEvent>;

export const LogWebVitalRequest = z.object({
  events: z.array(WebVitalEvent),
  country: z.string().optional(),
  region: z.string().optional(),
  city: z.string().optional(),
});
export type LogWebVitalRequest = z.infer<typeof LogWebVitalRequest>;

export const LogWebVital = {
  path: "/metrics/web-vital",
  method: "POST",
  request: LogWebVitalRequest,
  response: z.void(),
} as const satisfies Endpoint;

export type LogWebVital = typeof LogWebVital;
