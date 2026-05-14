"use client";

import { useTheme } from "@/hooks/use-theme";

export function ThemeToggle() {
  const { theme, toggleTheme } = useTheme();
  const isDark = theme === "dark";

  return (
    <button
      type="button"
      onClick={toggleTheme}
      className="button-secondary inline-flex items-center gap-3 rounded-full px-4 py-3 text-left transition-colors duration-200 hover:bg-surface"
      aria-label={
        isDark ? "Switch to light mode" : "Switch to dark mode"
      }
      aria-pressed={isDark}
    >
      <span
        className={`flex h-9 w-9 items-center justify-center rounded-full text-sm font-semibold ${
          isDark
            ? "bg-[linear-gradient(135deg,var(--meteor-gold),var(--meteor-nova))] text-meteor-midnight"
            : "bg-[linear-gradient(135deg,var(--meteor-cyan),#9cf7fb)] text-meteor-midnight"
        }`}
        aria-hidden="true"
      >
        {isDark ? "M" : "S"}
      </span>
      <span className="flex flex-col">
        <span className="text-[11px] font-semibold uppercase tracking-[0.24em] text-copy-soft">
          Theme
        </span>
        <span className="text-sm font-semibold text-meteor-ink">
          {isDark ? "Dark mode" : "Light mode"}
        </span>
      </span>
    </button>
  );
}
