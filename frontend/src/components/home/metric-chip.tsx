import type { HeroMetric } from "@/lib/design-system";

export function MetricChip({ label, value, detail }: HeroMetric) {
  return (
    <article className="glass-panel rounded-[1.5rem] p-5">
      <p className="text-3xl font-semibold tracking-[-0.04em] text-meteor-ink">
        {value}
      </p>
      <p className="mt-2 text-sm font-semibold uppercase tracking-[0.2em] text-copy-soft">
        {label}
      </p>
      <p className="mt-3 text-sm leading-6 text-copy-muted">{detail}</p>
    </article>
  );
}
