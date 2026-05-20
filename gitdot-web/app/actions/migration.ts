"use server";

import type {
  GitHubInstallationResource,
  GitHubRepositoryResource,
  MigrationResource,
} from "gitdot-api";
import {
  ApiError,
  getGithubAppInstallUrl,
  getMigration,
  listInstallationRepositories,
  listInstallations,
  listMigrations,
  migrateGitHubRepositories,
} from "@/dal";

export async function listInstallationsAction(): Promise<
  GitHubInstallationResource[]
> {
  const result = await listInstallations();
  return result?.data ?? [];
}

export async function getGithubAppInstallUrlAction(
  action: "migration" | "onboarding",
): Promise<{ url: string } | { error: string }> {
  const result = await getGithubAppInstallUrl(action);
  if (!result) {
    return { error: "failed to get github app install url" };
  }
  return { url: result.install_url };
}

export async function listInstallationRepositoriesAction(
  installationId: number,
): Promise<GitHubRepositoryResource[]> {
  return (await listInstallationRepositories(installationId)) ?? [];
}

export async function listMigrationsAction(): Promise<MigrationResource[]> {
  const result = await listMigrations();
  return result?.data ?? [];
}

export async function getMigrationAction(
  number: number,
): Promise<MigrationResource | null> {
  return await getMigration(number);
}

export type MigrateGitHubRepositoriesPayload = {
  installationId: number;
  origin: string;
  originType: string;
  destination: string;
  destinationType: string;
  repositories: { name: string; id: number }[];
  readonly: boolean;
};

export type MigrateGithubRepositoriesResult =
  | { migration: MigrationResource }
  | { error: string };

export async function migrateGitHubRepositoriesAction(
  payload: MigrateGitHubRepositoriesPayload,
): Promise<MigrateGithubRepositoriesResult> {
  if (!payload.destination || payload.repositories.length === 0) {
    return { error: "Destination and repositories are required" };
  }

  try {
    const migration = await migrateGitHubRepositories(
      payload.installationId,
      payload.origin,
      payload.originType,
      payload.destination,
      payload.destinationType,
      payload.repositories,
      payload.readonly,
    );
    if (!migration) {
      return { error: "Failed to start migration" };
    }
    return { migration };
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "Failed to start migration",
    };
  }
}
