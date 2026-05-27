import {
  type BundledLanguage,
  getSingletonHighlighter,
  type Highlighter,
  type ShikiTransformer,
} from "shiki";

/**
 * shiki short hands (e.g., shiki.codeToHast) internally manages a singleton instance and lazily loads themes and bundles as required
 *
 * that is all what we'd like to leverage as well, with the exception that we want to inject our own custom themes when requested
 * so we consolidate on the following API for our own internal usage
 */
const VITESSE_THEMES = {
  light: "vitesse-light",
  dark: "vitesse-dark",
} as const;

export async function getHighlighter(
  lang: BundledLanguage | undefined,
  theme: "vitesse" | "gitdot",
): Promise<Highlighter> {
  const highlighter = await getSingletonHighlighter();
  const themesToLoad: ("vitesse-light" | "vitesse-dark" | "gitdot-light")[] =
    theme === "vitesse" ? ["vitesse-light", "vitesse-dark"] : ["gitdot-light"];

  for (const t of themesToLoad) {
    if (highlighter.getLoadedThemes().includes(t)) continue;
    if (t === "gitdot-light") {
      const gitdotLight = (await import("./themes/gitdot-light")).default;
      await highlighter.loadTheme(gitdotLight);
    } else if (t === "vitesse-light") {
      const vitesseLight = (await import("@shikijs/themes/vitesse-light"))
        .default;
      await highlighter.loadTheme(vitesseLight);
    } else if (t === "vitesse-dark") {
      const vitesseDark = (await import("./themes/vitesse-dark")).default;
      await highlighter.loadTheme(vitesseDark);
    }
  }

  if (lang && !highlighter.getLoadedLanguages().includes(lang)) {
    await highlighter.loadLanguage(lang);
  }
  return highlighter;
}

export async function fileToHast(
  content: string,
  lang: BundledLanguage | undefined,
  theme: "vitesse" | "gitdot",
  transformers: ShikiTransformer[],
) {
  const highlighter = await getHighlighter(lang, theme);
  if (theme === "vitesse") {
    return highlighter.codeToHast(content, {
      lang: lang ?? "plaintext",
      themes: VITESSE_THEMES,
      defaultColor: "light",
      transformers,
    });
  }

  return highlighter.codeToHast(content, {
    lang: lang ?? "plaintext",
    theme: "gitdot-light",
    transformers,
  });
}
