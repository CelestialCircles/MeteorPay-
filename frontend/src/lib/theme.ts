export type ThemeMode = "light" | "dark";

export const THEME_STORAGE_KEY = "meteorpay-theme";
export const THEME_MEDIA_QUERY = "(prefers-color-scheme: dark)";

export function getThemeBootstrapScript() {
  return `
    (() => {
      const storageKey = "${THEME_STORAGE_KEY}";
      const mediaQuery = "${THEME_MEDIA_QUERY}";
      const root = document.documentElement;
      const storedTheme = window.localStorage.getItem(storageKey);
      const theme =
        storedTheme === "light" || storedTheme === "dark"
          ? storedTheme
          : window.matchMedia(mediaQuery).matches
            ? "dark"
            : "light";

      root.classList.toggle("dark", theme === "dark");
      root.dataset.theme = theme;
      root.style.colorScheme = theme;
    })();
  `;
}
