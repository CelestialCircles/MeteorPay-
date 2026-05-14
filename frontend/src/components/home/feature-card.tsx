import type { DashboardHighlight } from "@/lib/design-system";

export function FeatureCard({
  eyebrow,
  title,
  description,
}: DashboardHighlight) {
  return (
    <article className="glass-panel rounded-[1.75rem] p-6 transition-transform duration-200 hover:-translate-y-1">
      <p className="text-sm font-semibold uppercase tracking-[0.24em] text-copy-soft">
        {eyebrow}
      </p>
      <h3 className="mt-4 text-xl font-semibold text-meteor-ink">{title}</h3>
      <p className="mt-3 text-sm leading-7 text-copy-muted">{description}</p>
    </article>
  );
}
