import { z } from "zod";
import { GitHubAppInstallUrlResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const GetGitHubAppInstallUrlRequest = z.object({
  action: z.enum(["migration", "onboarding"]),
});
export type GetGitHubAppInstallUrlRequest = z.infer<
  typeof GetGitHubAppInstallUrlRequest
>;

export const GetGitHubAppInstallUrlResponse = GitHubAppInstallUrlResource;
export type GetGitHubAppInstallUrlResponse = z.infer<
  typeof GetGitHubAppInstallUrlResponse
>;

export const GetGitHubAppInstallUrl = {
  path: "/migration/github/install-url",
  method: "GET",
  request: GetGitHubAppInstallUrlRequest,
  response: GetGitHubAppInstallUrlResponse,
} as const satisfies Endpoint;
export type GetGitHubAppInstallUrl = typeof GetGitHubAppInstallUrl;
