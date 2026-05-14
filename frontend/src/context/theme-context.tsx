"use client";

import {
  THEME_MEDIA_QUERY,
  THEME_STORAGE_KEY,
  type ThemeMode,
} from "@/lib/theme";
import {
  createContext,
  useEffect,
  useState,
  type Dispatch,
  type SetStateAction,
} from "react";

type ThemeContextValue = {
  theme: ThemeMode;
  setTheme: Dispatch<SetStateAction<ThemeMode>>;
  toggleTheme: () => void;
};

function resolveInitialTheme(): ThemeMode {
  if (typeof document !== "undefined") {
    const documentTheme = document.documentElement.dataset.theme;
    if (documentTheme === "light" || documentTheme === "dark") {
      return documentTheme;
    }
  }

  if (typeof window === "undefined") {
    return "dark";
  }

  const storedTheme = window.localStorage.getItem(THEME_STORAGE_KEY);
  if (storedTheme === "light" || storedTheme === "dark") {
    return storedTheme;
  }

  return window.matchMedia(THEME_MEDIA_QUERY).matches
    ? "dark"
    : "light";
}

export const ThemeContext = createContext<ThemeContextValue | undefined>(
  undefined,
);

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [theme, setTheme] = useState<ThemeMode>(resolveInitialTheme);

  useEffect(() => {
    const root = document.documentElement;
    root.classList.toggle("dark", theme === "dark");
    root.dataset.theme = theme;
    root.style.colorScheme = theme;
    window.localStorage.setItem(THEME_STORAGE_KEY, theme);
  }, [theme]);

  useEffect(() => {
    function syncTheme(event: StorageEvent) {
      if (event.key !== THEME_STORAGE_KEY) {
        return;
      }

      if (event.newValue === "light" || event.newValue === "dark") {
        setTheme(event.newValue);
      }
    }

    window.addEventListener("storage", syncTheme);

    return () => {
      window.removeEventListener("storage", syncTheme);
    };
  }, []);

  return (
    <ThemeContext.Provider
      value={{
        theme,
        setTheme,
        toggleTheme: () =>
          setTheme((currentTheme) =>
            currentTheme === "dark" ? "light" : "dark",
          ),
      }}
    >
      {children}
    </ThemeContext.Provider>
  );
}
