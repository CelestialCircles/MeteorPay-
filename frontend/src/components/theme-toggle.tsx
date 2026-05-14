"use client";

import { useTheme } from "@/hooks/use-theme";

export function ThemeToggle() {
  const { toggleTheme } = useTheme();

  return (
    <button
      type="button"
      onClick={toggleTheme}
      className="button-secondary inline-flex items-center gap-3 rounded-full px-4 py-3 text-xs font-semibold uppercase tracking-[0.22em] text-copy-muted transition-colors duration-200 hover:bg-surface"
      aria-label="Toggle color mode"
    >
      <span className="h-2.5 w-2.5 rounded-full bg-[linear-gradient(135deg,var(--meteor-cyan),var(--meteor-gold))]" />
      Toggle theme
    </button>
  );
}
